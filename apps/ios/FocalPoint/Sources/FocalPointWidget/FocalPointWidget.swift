import WidgetKit
import SwiftUI

@main
struct FocalPointWidgetBundle: WidgetBundle {
    var body: some Widget {
        CreditsWidget()
        TodayBriefWidget()
        if #available(iOS 16.1, *) {
            FocusSessionLiveActivity()
        }
    }
}
