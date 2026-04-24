import Foundation

/// Emotion/tone for Coachy responses.
enum CoachyEmotion: String {
    case happy
    case excited
    case proud
    case neutral
    case concerned
    case disappointed
    case focused
    case tired
    case encouraging
    case supportive
}

/// State snapshot used to generate a natural-language coaching response.
struct CoachyState {
    let taskCount: Int
    let completedToday: Int
    let credits: Int
    let streakDays: Int
    let ritualStatus: String  // e.g., "Morning Brief pending", "all caught up"
    let weekdayIndex: Int  // 0 = Sunday, 6 = Saturday
    let timeOfDay: TimeOfDay  // Morning, Afternoon, Evening

    enum TimeOfDay {
        case morning    // 5am–12pm
        case afternoon  // 12pm–5pm
        case evening    // 5pm–12am
        case night      // 12am–5am

        static var current: TimeOfDay {
            let hour = Calendar.current.component(.hour, from: Date())
            switch hour {
            case 5..<12: return .morning
            case 12..<17: return .afternoon
            case 17..<24: return .evening
            default: return .night
            }
        }
    }
}

/// Pure function mapping state → natural-language response with emotion hint.
struct CoachyResponseSynth {
    static func generateResponse(state: CoachyState) -> (text: String, emotion: CoachyEmotion) {
        // Rotate through 8 different response phrasings based on day of week.
        let phraseIndex = state.weekdayIndex % 8

        let overallStatus: String
        if state.taskCount == 0 {
            overallStatus = "You haven't added any tasks yet."
        } else if state.completedToday == state.taskCount {
            overallStatus = "You've crushed all your tasks today!"
        } else {
            let remaining = state.taskCount - state.completedToday
            overallStatus = "You have \(remaining) task\(remaining > 1 ? "s" : "") left."
        }

        let creditsStatus: String
        if state.credits > 500 {
            creditsStatus = "You're sitting pretty with \(state.credits) credits."
        } else if state.credits > 100 {
            creditsStatus = "You've got \(state.credits) credits in the bank."
        } else if state.credits > 0 {
            creditsStatus = "You're low on credits—only \(state.credits) left."
        } else {
            creditsStatus = "You're out of credits. Time to earn some!"
        }

        let ritualHint: String
        if state.ritualStatus.contains("pending") {
            ritualHint = " You haven't done your \(state.ritualStatus.lowercased()) yet."
        } else if state.ritualStatus.contains("caught up") {
            ritualHint = " Rituals all caught up!"
        } else {
            ritualHint = ""
        }

        let streakHint: String
        if state.streakDays > 1 {
            streakHint = " You're on a \(state.streakDays)-day streak—keep it up!"
        } else if state.streakDays == 1 {
            streakHint = " Start building your streak today!"
        } else {
            streakHint = ""
        }

        let baseResponse: String
        let emotion: CoachyEmotion

        switch phraseIndex {
        case 0:
            baseResponse = "\(state.timeOfDay == .morning ? "Good morning, Koosha." : "Hey there.") \(overallStatus) \(creditsStatus)\(ritualHint)"
            emotion = state.completedToday > 0 ? .encouraging : .neutral

        case 1:
            baseResponse = "Status update: \(overallStatus) Credits: \(state.credits).\(ritualHint)\(streakHint)"
            emotion = state.credits > 200 ? .happy : .concerned

        case 2:
            baseResponse = "Let's see... you've got \(state.taskCount) on your plate, and \(state.completedToday) done so far. \(creditsStatus)\(streakHint)"
            emotion = state.completedToday > state.taskCount / 2 ? .proud : .supportive

        case 3:
            baseResponse = "\(creditsStatus) \(overallStatus)\(ritualHint) Ready to get more done?"
            emotion = .focused

        case 4:
            baseResponse = "You're running on \(state.credits) credits. \(overallStatus)\(streakHint)"
            emotion = state.credits > 100 ? .happy : .concerned

        case 5:
            baseResponse = "Here's what I see: \(overallStatus) You've got \(state.credits) credits.\(ritualHint)"
            emotion = state.completedToday == state.taskCount ? .excited : .neutral

        case 6:
            baseResponse = "\(state.timeOfDay == .evening ? "Wrapping up the day" : "Moving right along"). \(overallStatus) Balance: \(state.credits).\(streakHint)"
            emotion = state.streakDays > 3 ? .proud : .encouraging

        case 7:
            baseResponse = "Quick check: \(state.completedToday) of \(state.taskCount) tasks done. \(creditsStatus)\(ritualHint)"
            emotion = state.taskCount > 0 && state.completedToday == 0 ? .concerned : .neutral

        default:
            baseResponse = "\(overallStatus) \(creditsStatus)"
            emotion = .neutral
        }

        return (baseResponse.trimmingCharacters(in: .whitespaces), emotion)
    }
}
