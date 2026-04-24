#if canImport(SwiftUI)
import SwiftUI

/// Renders markdown text inline using SwiftUI's native AttributedString markdown
/// parser (iOS 15+). Does not require external dependencies; parses basic markdown
/// formatting (bold, italic, links, headings, lists).
public struct MarkdownView: View {
    let text: String
    let fontSize: CGFloat
    let foregroundColor: Color
    let linkColor: Color

    public init(
        text: String,
        fontSize: CGFloat = 16,
        foregroundColor: Color = Color.app.foreground,
        linkColor: Color = Color.app.accent
    ) {
        self.text = text
        self.fontSize = fontSize
        self.foregroundColor = foregroundColor
        self.linkColor = linkColor
    }

    public var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 12) {
                if let attributedString = parseMarkdown(text) {
                    Text(attributedString)
                        .font(.system(size: fontSize, design: .default))
                        .foregroundColor(foregroundColor)
                        .tint(linkColor)
                        .lineSpacing(4)
                } else {
                    Text(text)
                        .font(.system(size: fontSize, design: .default))
                        .foregroundColor(foregroundColor)
                        .lineSpacing(4)
                }
            }
            .padding(16)
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    /// Parse markdown into AttributedString. Handles:
    /// - **bold** or __bold__
    /// - *italic* or _italic_
    /// - # Headings through ######
    /// - [links](url)
    /// - Lists (- or *)
    private func parseMarkdown(_ markdown: String) -> AttributedString? {
        // iOS 15.1+ supports markdown via AttributedString initializer.
        // If markdown parsing is not available, this gracefully falls back to plain text.
        do {
            return try AttributedString(markdown: markdown)
        } catch {
            // Markdown parsing failed; fall back to plain text rendering.
            return nil
        }
    }
}

#Preview {
    MarkdownView(
        text: """
        # Example Markdown

        This is **bold** text and this is *italic* text.

        - List item 1
        - List item 2

        [Link to example](https://example.com)
        """,
        fontSize: 14
    )
}
#endif
