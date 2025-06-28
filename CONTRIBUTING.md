# Contributing to pacemanbot
We encourage contributions to the bots (`PaceManBot`, `PaceManBot1.15`, `PaceManBot1.7` and `PaceManBotAA`) either through issues/suggestions or code.

## Issues
You can create an issue [here](https://github.com/paceman-mcsr/pacemanbot/issues). There is no specific format that needs to be followed for an issue but try to provide as much information as possible to make sure the contributors/maintainers can understand what is happening and can help you out.

You may also open a suggestion thread with the `Bot` tag in the [discord](https://discord.gg/t63gGSWvdV) and the maintainers/contributors would then be adding that suggestion to the issue tracker manually. Maintainers/contributors can also provide a link to the suggestion thread on discord while creating the issue.

## Development
### Setup
To setup your system for development of pacemanbot, all you require is an editor (any of your choice, just make sure you don't commit editor configs into the PR like `.vscode`, `.idea` etc), Git installed, a Github account and the Rust compiler & its components (`cargo` and `rustfmt`) installed.

You can look at the following links to set these up:
1. Git: Download from [here](https://git-scm.com/downloads).
2. Rust: Follow the steps [here](https://www.rust-lang.org/tools/install).
3. Cargo: Run in terminal/cmd prompt: `rustup component add cargo`
4. Rustfmt: Run in terminal/cmd prompt: `rustup component add rustfmt`

### Assign yourself to an issue
Make sure to communicate clearly to the maintainer(s) that you would be working on the issue by commenting in the issue thread. Maintainers may also assign the issue to you through Github afterwards.

### Setup the test bot
You can add these test bot(s) to any test server you want:
1. [1.16 bot](https://discord.com/oauth2/authorize?client_id=1187382354004688916).
2. [1.15 bot]().
3. [1.7 bot]().
4. [AA bot]().

This is required when the CI runs integration tests on your Pull Request, you can see the outputs that each bot would produce for the different splits in each version/category.

### Begin developing
Fork the repo (make sure to disable cloning only the `main` branch if you are working on `PaceManBot1.15`, `PaceManBot1.7` and `PaceManBotAA`), create a feature branch (typically named `feat/<feature_name>` or `fix/<fix_name>` if it is a bug fix, no restriction on the naming).
Clone your fork into your local system and start working inside of the feature branch.

### Local testing
If you are working on any bot and the work is in functionality that is independent of paceman.gg, you can setup your own test bot in a test server and test the bot locally.

Create a `.env` file at the project root like the following:
```
BOT_TOKEN=<your test bot token goes here>
API_AUTH_KEY="" # Leave it blank.
WS_HOST="" # Leave it blank.
WS_URL="" # Leave it blank
WEBHOOK_URL="" # You may create a discord webhook for error logging. But it is not discussed as it is out of scope.
```
and run the bot by running (while using [GNU Make](https://www.gnu.org/software/make/)),
```bash
make run
```
or manually by running,
```bash
cargo run
```

You can now go to your test server and run the commands you want to test.

### Commit styling
Commit styling is not strict. As long as your commit is of the format `<action>: <description>` format where `<action>` can take the values `feat`, `fix`, `docs`, `chore`, `revert` and `ci` and the `<description>` can be a short (not more than 80 characters) description of what the commit is doing.
Maintainers are to follow this strictly while merging commits to the main branches (`main`, `1.15`, `1.7` and `AA`).

Eg: `fix: reconnection issues`, `feat: /migrate command`, `revert: 92e2692 and 26884d3`, `ci: include unit test runners`, `docs: reformat README`, `chore: bump serenity-rs to v0.12.0`, etc.

If the changes you are making is a breaking change (i.e. Bot's users would need to do some changes on their existing configuration when deployed), you should append the `<action>` with a `!`.

Eg: `feat!: /whitelist by minutes and seconds`, `feat!: /whitelist by UUIDs instead of IGNs`, etc.

If you are working on changes for the docs, you can also skip running the CI tests by appending `skip-checks: true` to the trailers of your commit message (after leaving two blank lines).

Eg: 
```
docs: add contributor docs


skip-checks: true
```

### Code formatting
Format all your code using `rustfmt` by running,
```bash
cargo fmt -- --emit files
```
before you make commits to your PR.

### Fixing Lint Warnings
Fix all lint warnings from `rustc` by running,
```bash
cargo fix
```
before you make commits to your PR.

### Pull Requests
Once you are done, open a Pull Request to the appropriate base branch in this repo (`main` for the 1.16 bot, `1.15` for the 1.15 bot, `1.7` for the 1.7 bot and `AA` for the AA bot).

You may also open the Pull Request as a **draft** if your work is still in progress, but you would still get the Actions run on all your new commits.
Wait until the maintainer(s) review your code and merge it!
