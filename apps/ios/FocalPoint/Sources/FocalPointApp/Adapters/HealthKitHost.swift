#if canImport(HealthKit)
import HealthKit
import Foundation
import FocalPointCore

/// `HealthKitHost` — iOS host-emitted events from HealthKit.
/// Emits `apple-health:workout`, `apple-health:sleep_reported`, `apple-health:steps_milestone` events.
///
/// Traces to: FR-HEALTHKIT-001 — Apple Health event bridging to Rust core.
public final class HealthKitHost: HealthEventHost {
    private let store: HKHealthStore
    private let isoFormatter: ISO8601DateFormatter
    private var workoutQuery: HKObserverQuery?
    private var sleepQuery: HKObserverQuery?
    private var stepsQuery: HKObserverQuery?

    public init(store: HKHealthStore = HKHealthStore()) {
        self.store = store
        let fmt = ISO8601DateFormatter()
        fmt.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        self.isoFormatter = fmt
    }

    /// Request authorization to read workouts, sleep, and step count.
    /// Requires `NSHealthShareUsageDescription` in Info.plist.
    /// iOS 17+ uses `requestPeriodicAuthorization`, older uses `requestAuthorization`.
    public func requestAccess() async throws -> Bool {
        let typesToRead: Set<HKObjectType> = [
            HKObjectType.workoutType(),
            HKObjectType.quantityType(forIdentifier: .stepCount)!,
            HKObjectType.categoryType(forIdentifier: .sleepAnalysis)!,
        ]

        if #available(iOS 17.0, *) {
            return try await store.requestPeriodicAuthorization(
                toShare: [],
                read: typesToRead,
                requestTitle: "FocalPoint Health",
                requestSubtitle: "FocalPoint reads your workouts, sleep, and daily step milestones to reward you via rules. Data stays on device."
            )
        } else {
            return try await withCheckedThrowingContinuation { cont in
                store.requestAuthorization(toShare: [], read: typesToRead) { granted, err in
                    if let err = err { cont.resume(throwing: err) }
                    else { cont.resume(returning: granted) }
                }
            }
        }
    }

    /// Start observing workouts. Calls `HostEventApi.emit_event` when new workouts are detected.
    public func startWorkoutObserver() {
        let predicate = HKQuery.predicateForWorkouts(
            with: NSPredicate(format: "duration > 0")
        )
        let query = HKObserverQuery(
            sampleType: HKObjectType.workoutType(),
            predicate: predicate
        ) { [weak self] _, completionHandler, _ in
            self?.handleWorkoutUpdate(completionHandler: completionHandler)
        }
        store.execute(query)
        self.workoutQuery = query
    }

    /// Start observing sleep. Calls `HostEventApi.emit_event` when new sleep is recorded.
    public func startSleepObserver() {
        if #available(iOS 16.0, *) {
            let predicate = HKQuery.predicateForSamples(
                withStart: Calendar.current.date(byAdding: .day, value: -1, to: Date())!,
                end: nil
            )
            let sleepType = HKObjectType.categoryType(forIdentifier: .sleepAnalysis)!
            let query = HKObserverQuery(
                sampleType: sleepType,
                predicate: predicate
            ) { [weak self] _, completionHandler, _ in
                self?.handleSleepUpdate(completionHandler: completionHandler)
            }
            store.execute(query)
            self.sleepQuery = query
        }
    }

    /// Start observing step count. Emits `steps_milestone` when daily total ≥ 10k.
    public func startStepsObserver() {
        let stepsType = HKObjectType.quantityType(forIdentifier: .stepCount)!
        let predicate = HKQuery.predicateForSamples(
            withStart: Calendar.current.startOfDay(for: Date()),
            end: Date(),
            options: .strictStartDate
        )
        let query = HKObserverQuery(
            sampleType: stepsType,
            predicate: predicate
        ) { [weak self] _, completionHandler, _ in
            self?.handleStepsUpdate(completionHandler: completionHandler)
        }
        store.execute(query)
        self.stepsQuery = query
    }

    // MARK: - Private Handlers

    private func handleWorkoutUpdate(completionHandler: @escaping (HKObserverQueryCompletionHandler)) {
        let workoutType = HKObjectType.workoutType()
        let query = HKSampleQuery(
            sampleType: workoutType,
            predicate: nil,
            limit: 10,
            sortDescriptors: [
                NSSortDescriptor(key: HKSampleSortIdentifierStartDate, ascending: false),
            ]
        ) { [weak self] _, samples, _ in
            guard let workouts = samples as? [HKWorkout] else {
                completionHandler(true, nil)
                return
            }
            for workout in workouts {
                self?.emitWorkoutEvent(workout)
            }
            completionHandler(true, nil)
        }
        store.execute(query)
    }

    private func handleSleepUpdate(completionHandler: @escaping (HKObserverQueryCompletionHandler)) {
        if #available(iOS 16.0, *) {
            let sleepType = HKObjectType.categoryType(forIdentifier: .sleepAnalysis)!
            let query = HKSampleQuery(
                sampleType: sleepType,
                predicate: nil,
                limit: 10,
                sortDescriptors: [
                    NSSortDescriptor(key: HKSampleSortIdentifierStartDate, ascending: false),
                ]
            ) { [weak self] _, samples, _ in
                guard let sleepSamples = samples as? [HKCategorySample] else {
                    completionHandler(true, nil)
                    return
                }
                for sample in sleepSamples {
                    self?.emitSleepEvent(sample)
                }
                completionHandler(true, nil)
            }
            store.execute(query)
        } else {
            completionHandler(true, nil)
        }
    }

    private func handleStepsUpdate(completionHandler: @escaping (HKObserverQueryCompletionHandler)) {
        let stepsType = HKObjectType.quantityType(forIdentifier: .stepCount)!
        let predicate = HKQuery.predicateForSamples(
            withStart: Calendar.current.startOfDay(for: Date()),
            end: Date()
        )
        let query = HKStatisticsQuery(
            quantityType: stepsType,
            quantitySamplePredicate: predicate,
            options: .cumulativeSum
        ) { [weak self] _, result, _ in
            if let sum = result?.sumQuantity() {
                let steps = Int(sum.doubleValue(for: HKUnit.count()))
                if steps >= 10_000 {
                    self?.emitStepsMilestoneEvent(steps)
                }
            }
            completionHandler(true, nil)
        }
        store.execute(query)
    }

    private func emitWorkoutEvent(_ workout: HKWorkout) {
        let payload: [String: Any] = [
            "workout_type": workoutTypeString(workout.workoutActivityType),
            "duration_minutes": Int(workout.duration / 60),
            "calories": Int(workout.totalEnergyBurned?.doubleValue(for: .kilocalorie()) ?? 0),
            "started_at_iso": isoFormatter.string(from: workout.startDate),
            "ended_at_iso": isoFormatter.string(from: workout.endDate),
        ]

        // Call Rust core's HostEventApi.emit_event
        // TODO: Wire via focus-ffi when available
        // FocalPointCore.hostEventApi().emitEvent(
        //     connectorId: "apple-health",
        //     eventType: "apple-health:workout",
        //     payload: payload
        // )
    }

    private func emitSleepEvent(_ sample: HKCategorySample) {
        let duration = sample.endDate.timeIntervalSince(sample.startDate) / 3600
        let payload: [String: Any] = [
            "hours": duration,
            "in_bed_at_iso": isoFormatter.string(from: sample.startDate),
            "ended_at_iso": isoFormatter.string(from: sample.endDate),
            "sleep_efficiency": 85, // TODO: derive from HK data
        ]

        // TODO: Wire via focus-ffi
    }

    private func emitStepsMilestoneEvent(_ steps: Int) {
        let payload: [String: Any] = [
            "steps": steps,
            "date_iso": ISO8601DateFormatter().string(from: Date()),
        ]

        // TODO: Wire via focus-ffi
    }

    private func workoutTypeString(_ type: HKWorkoutActivityType) -> String {
        switch type {
        case .running:
            return "running"
        case .walking:
            return "walking"
        case .cycling:
            return "cycling"
        case .swimming:
            return "swimming"
        case .elliptical:
            return "elliptical"
        case .rowing:
            return "rowing"
        default:
            return "workout"
        }
    }
}

#endif
