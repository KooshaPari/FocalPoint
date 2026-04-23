#if canImport(SwiftUI) && canImport(UIKit) && canImport(AVFoundation)
import SwiftUI
import UIKit
import AVFoundation

/// Thin SwiftUI wrapper around AVCaptureMetadataOutput for QR payloads.
/// Requires the `NSCameraUsageDescription` Info.plist key in the eventual
/// Xcode project (see README).
public struct QRScannerView: UIViewControllerRepresentable {
    public typealias OnScan = (String) -> Void

    public let onScan: OnScan

    public init(onScan: @escaping OnScan) {
        self.onScan = onScan
    }

    public func makeCoordinator() -> Coordinator {
        Coordinator(onScan: onScan)
    }

    public func makeUIViewController(context: Context) -> QRScannerViewController {
        let vc = QRScannerViewController()
        vc.delegate = context.coordinator
        return vc
    }

    public func updateUIViewController(_ uiViewController: QRScannerViewController, context: Context) {}

    public final class Coordinator: NSObject, QRScannerViewControllerDelegate {
        let onScan: OnScan
        public init(onScan: @escaping OnScan) { self.onScan = onScan }

        public func qrScanner(_ vc: QRScannerViewController, didScan payload: String) {
            onScan(payload)
        }
    }
}

public protocol QRScannerViewControllerDelegate: AnyObject {
    func qrScanner(_ vc: QRScannerViewController, didScan payload: String)
}

public final class QRScannerViewController: UIViewController, AVCaptureMetadataOutputObjectsDelegate {
    public weak var delegate: QRScannerViewControllerDelegate?
    private let session = AVCaptureSession()
    private var preview: AVCaptureVideoPreviewLayer?

    public override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black
        configureSession()
    }

    public override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        if !session.isRunning { session.startRunning() }
    }

    public override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        if session.isRunning { session.stopRunning() }
    }

    public override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        preview?.frame = view.bounds
    }

    private func configureSession() {
        guard let device = AVCaptureDevice.default(for: .video),
              let input = try? AVCaptureDeviceInput(device: device),
              session.canAddInput(input) else {
            return
        }
        session.addInput(input)

        let output = AVCaptureMetadataOutput()
        guard session.canAddOutput(output) else { return }
        session.addOutput(output)
        output.setMetadataObjectsDelegate(self, queue: .main)
        output.metadataObjectTypes = [.qr]

        let preview = AVCaptureVideoPreviewLayer(session: session)
        preview.videoGravity = .resizeAspectFill
        preview.frame = view.bounds
        view.layer.addSublayer(preview)
        self.preview = preview
    }

    public func metadataOutput(
        _ output: AVCaptureMetadataOutput,
        didOutput metadataObjects: [AVMetadataObject],
        from connection: AVCaptureConnection
    ) {
        for obj in metadataObjects {
            guard
                let readable = obj as? AVMetadataMachineReadableCodeObject,
                readable.type == .qr,
                let payload = readable.stringValue
            else { continue }
            delegate?.qrScanner(self, didScan: payload)
        }
    }
}
#endif
