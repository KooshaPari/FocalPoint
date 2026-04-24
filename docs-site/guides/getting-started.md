---
title: Getting Started with FocalPoint
description: Step-by-step guide to install, set up your first connector, create your first rule, and earn your first credits.
---

# Getting Started with FocalPoint

Welcome! This guide takes you from zero to your first credit reward in 10 steps. You'll install the app, connect a tool, write your first rule, trigger it, and celebrate your first unlock.

**Time commitment:** ~15 minutes (first launch + connector setup + rule creation). Subsequent rules take ~2 minutes each.

---

## Step 1: Install FocalPoint

### Option A: TestFlight (Recommended once available)

FocalPoint will ship on TestFlight for public beta testing. Until then, use Option B.

### Option B: Build from Source

If you're on a Mac with Xcode 15+ and a paid Apple Developer account:

```bash
# Clone the repo
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint

# Install dependencies
brew install rust task

# Build for iOS
cargo build --target aarch64-apple-ios

# Open Xcode and run on device
open apps/ios/FocalPoint.xcodeproj
```

**Prerequisites:**
- iOS 16+ device (iPhone or iPad)
- Xcode 15+
- Paid Apple Developer account ($99/year)
- **FamilyControls entitlement** approved by Apple (1–4 weeks lead time — apply early via your developer account)

See [Install on iOS](/getting-started/install-ios) for detailed setup instructions.

---

## Step 2: Launch and Complete Onboarding

When you open FocalPoint for the first time, you'll see:

1. **Welcome screen** ![](/assets/tutorial/01-welcome.png)
   - "FocalPoint keeps you focused." — description of the app
   - Tap "Get Started" to proceed
   - Coachy (the mascot) appears to guide you

2. **Permissions screen** ![](/assets/tutorial/02-permissions.png)
   - Grant FamilyControls access (required to enforce rules)
   - Grant Notification permission (for rule alerts and rewards)
   - Grant Health app permission (if using Apple Health connector)

3. **Welcome to the Wallet** ![](/assets/tutorial/03-wallet.png)
   - You start with **0 credits**
   - Your first credit arrives when a rule fires
   - Coachy explains: "Follow rules → Earn credits → Unlock rewards"

4. **Quick tour** ![](/assets/tutorial/04-tour.png)
   - Brief walkthrough of the main tabs:
     - **Activity** — Watch rules fire and events arrive in real time
     - **Rules** — Manage your automation rules
     - **Wallet** — Track credits and redeem rewards
     - **Connectors** — Add tools (GitHub, Canvas, Strava, etc.)

---

## Step 3: Connect Your First Tool (GitHub)

GitHub is the easiest connector to set up. You'll use it to trigger your first rule.

1. Open FocalPoint and tap **Connectors** (bottom tab)

2. Find "GitHub" and tap "Connect" ![](/assets/tutorial/05-connectors.png)

3. Tap "Authorize with GitHub" — your browser opens to GitHub's OAuth screen

4. Grant FocalPoint permission to:
   - Read your public repos
   - Read your commits and pull requests
   - **(No write access required)**

5. FocalPoint stores your GitHub token securely in your device keychain

6. You'll see **"GitHub connected"** with a green checkmark ![](/assets/tutorial/06-github-connected.png)

**What's happening under the hood:**
- FocalPoint syncs your GitHub activity every 60 seconds
- A background service listens for events: commits, PRs opened, PRs merged, etc.
- Each event is recorded as an **audit-chained event** in the local SQLite database

---

## Step 4: Wait for the First Heartbeat Sync

After connecting GitHub, FocalPoint begins syncing.

1. Open **Activity** tab ![](/assets/tutorial/07-activity-empty.png)

2. You'll see a loading indicator: "Syncing with GitHub..."

3. Wait ~60 seconds (the default heartbeat interval)

4. Your first event arrives! ![](/assets/tutorial/08-first-event.png)
   - Event type: `github:commit_pushed` or `github:pull_request_opened`
   - Timestamp: when the event occurred
   - Event details (repo, branch, author)

**Why 60 seconds?**
- FocalPoint respects API rate limits
- Syncing happens asynchronously in the background
- You're never blocked by network calls

---

## Step 5: Observe Events in Real Time

Now watch your Activity feed light up.

1. Keep **Activity** tab open

2. Make a test action in GitHub:
   - Create a new branch in one of your repos
   - Push a commit with `git push origin test-branch`
   - Or open and merge a small pull request

3. Within ~60 seconds, FocalPoint detects the event ![](/assets/tutorial/09-new-event.png)
   - The Activity tab updates
   - You see: "github:commit_pushed" with all details
   - Coachy waves: "Hey! I detected an event!"

**This is the core loop:**
> Connector → Event → Activity Feed → Rules watch this feed

---

## Step 6: Create Your First Rule (Three Paths)

Time to build logic! FocalPoint gives you three ways to express the same rule. Pick your favorite:

### Path A: Via CLI (Fastest for Developers)

Open Terminal and run:

```bash
focus rules add \
  --name "github-pr-reward" \
  --when "github:pull_request_merged" \
  --then "wallet:grant 10"
```

This creates a rule: "When a PR is merged on GitHub, grant 10 credits."

### Path B: Via FPL (Starlark — Most Powerful)

Create a file `~/.focalpoint/rules/github-pr-reward.fpl`:

```python
# FocalPoint PR Reward Rule (Starlark/FPL)

rule(
    id="github-pr-reward",
    name="Merged PR — 10 credits",
    trigger=on_event("github:pull_request_merged"),
    conditions=[
        # Bonus: only reward if NOT in a review period
        not_in_mode("review_window")
    ],
    actions=[
        grant_credits(amount=10, reason="PR merged"),
        notify(
            title="PR Merged!",
            body="You earned 10 credits. Nice work!"
        ),
    ],
    enabled=True
)
```

### Path C: Via Rule Builder (UI — Most Visual)

In FocalPoint, open **Rules** → **+ New Rule**:

1. **Name**: "Merged PR — 10 credits"
2. **When**: Choose "GitHub" → "Pull request merged" ![](/assets/tutorial/10-rule-builder-when.png)
3. **Conditions** (optional): Leave empty for now
4. **Then**: Choose "Grant credits" → Amount: 10 ![](/assets/tutorial/11-rule-builder-then.png)
5. **Tap "Save"** ![](/assets/tutorial/12-rule-saved.png)

**What's identical across all three:**
- All three compile to the same **canonical IR (Intermediate Representation)**
- The canonical IR is deterministic and hash-verifiable
- Each IR has a **SHA-256 fingerprint** that proves it's the same rule

Example IR hash: `sha256:a7f2b9e4c3d1f8a6b5c2e9d7a3f1b8c4e6a9d2f5c8b1e4a7d0f3c6a9e1b4`

---

## Step 7: Trigger Your First Rule

Now let's make your rule fire.

1. Open GitHub in your browser (not in FocalPoint)

2. Create a **test pull request**:
   - Go to one of your repos
   - Click "New pull request"
   - Create a minimal change (e.g., edit README, add a comment)
   - Open the PR

3. Merge the PR:
   - Click "Merge pull request" → Confirm

4. **Return to FocalPoint** and watch the magic:
   - The Activity tab updates within ~60 seconds
   - You see: `github:pull_request_merged`
   - Your rule triggers automatically
   - **Notification pops up**: "PR Merged! You earned 10 credits." ![](/assets/tutorial/13-notification.png)

5. Open **Rules** → Your rule name → **History**
   - You see the decision trace:
     - Event: `github:pull_request_merged`
     - Condition evaluation: "Passed" (or skipped if no conditions)
     - Action: `grant_credits(10)`
     - Audit hash: cryptographic proof of the action

---

## Step 8: Earn Your First Credits

Check your wallet to see the credit appear.

1. Tap **Wallet** tab ![](/assets/tutorial/14-wallet-updated.png)
   - **Credit balance**: Now shows **10 credits** (was 0)
   - **Recent activity**: "PR Merged — +10 credits"
   - A celebration animation plays

2. Swipe down to see the **transaction history**:
   - Timestamp
   - Event that triggered it
   - Amount
   - Rule name
   - Audit hash (immutable proof)

**Why is this powerful?**
- Every credit is **cryptographically audited** — you can prove no tampering occurred
- Rules are **deterministic** — the same event always produces the same outcome
- Credits are **non-fungible** — each one is linked to the rule that granted it

---

## Step 9: Unlock Your First Reward

FocalPoint comes with pre-loaded rewards. Use your credits to unlock one.

1. Tap **Wallet** → **Rewards** tab ![](/assets/tutorial/15-rewards.png)

2. Browse the default reward pack:
   - "Screen time boost: +1 hour" — 25 credits
   - "Skip focus session" — 15 credits
   - "Unlock one distraction" — 10 credits
   - "Focus mode grace period" — 20 credits

3. Tap a reward you want. For example: **"Unlock one distraction — 10 credits"**

4. Confirm the redemption:
   - FocalPoint shows: "You'll spend 10 credits"
   - You have exactly 10 credits
   - Tap **"Redeem"** ![](/assets/tutorial/16-redemption-confirm.png)

5. **Instant reward**:
   - Your balance drops to **0 credits**
   - For the next 5 minutes, one blocked app is unlocked
   - Notification: "Reward unlocked! 5 minutes of grace." ![](/assets/tutorial/17-reward-active.png)
   - Coachy celebrates with a fanfare

6. Check **Wallet** → **Redemption history**:
   - You see the audit trail
   - Timestamp, reward name, credits spent, new balance

---

## Step 10: Celebrate (and Keep Going!)

You just completed the FocalPoint core loop:

```
Connect → Sync → Create Rule → Trigger → Earn → Redeem → Celebrate
```

### What's Next?

1. **Create more rules** to match your real workflows:
   - GitHub: Reward merged PRs, reviewed PRs, issue creations
   - Canvas: Reward assignments submitted on time
   - Apple Health: Reward workouts completed
   - Strava: Reward PRs and long runs
   - Or combine: "PR + health check-in together → extra bonus"

2. **Explore the Rule Builder**:
   - Try conditional logic: "Reward only on weekdays"
   - Add escalations: "After 3 rules in a day, double the credits"
   - Create penalties: "Missed focus session? -5 credits"

3. **Share your rules**:
   - Export as a template
   - Contribute to the community rule pack
   - Help others with the same workflow

4. **Read deeper docs**:
   - [Your First Rule](/guides/your_first_rule) — 3 parallel surfaces (CLI, FPL, Builder) with the same rule
   - [Write a Rule](/rules/) — Full rule DSL reference
   - [Five-Minute Tour](/guides/five_minute_tour) — Visual skimmer's guide

### FAQs

**Q: Can I create rules without GitHub?**
A: Yes! FocalPoint ships with Canvas, Strava, Apple Health, Todoist, and more. Pick any connector.

**Q: What if my rule never fires?**
A: Check Activity → make sure events are arriving. If not, re-authorize the connector.

**Q: Can I earn credits other ways?**
A: Yes — any rule action that includes `grant_credits(...)` adds to your wallet. Penalties deduct.

**Q: Is this data private?**
A: Yes. Everything runs locally on your device. Connectors sync events via their official APIs, but FocalPoint never sends your rules elsewhere.

---

## You're Done!

Coachy is proud. Go earn some credits. 🎉

Questions? [Join the Discord](https://discord.gg/focalpoint) or [open an issue](https://github.com/KooshaPari/FocalPoint/issues).
