# pacemanbot
A Discord bot to query paceman.gg, ping pace-roles and assign pace-roles to users.

# Usage (in your own Discord Server)
- First, use [this link](https://discord.com/oauth2/authorize?client_id=1321540208377991259) to add the bot to your discord server.
- Before we do anything, we would want to disable everyone from accessing the bot commands as they are meant for admins.
- Go to your server settings and go into the `Integrations` tab. Under there, select `PaceManBot1.15` and just disable the option that says `@everyone` under `Role & Members`. You can add in like an `admin` role and enable that role to be able to use this role in this same tab.
- Now in your discord server, create a new channel named `#other-versions`.
- This is where your pace pings will go. So make sure to give the role `PaceManBot1.15` all the necessary perms for sending, reading and mentioning roles in that channel.
- And then, create a new channel named `#pacemanbot-runner-names-115`.
- Then do
```
/whitelist <action> <ign> [<enter_nether> <enter_fortress> <blind> <eye_spy> <enter_end> <finish>]
```
- `<action>` takes the values: `add_or_update` or `remove`
- `add_or_update` either adds a new runner (if they aren't already in the config) or updates an existing runner's splits to the new splits that will be specified.
- `remove` removes a runner (if they exist already in the config).
- Note here that all structure/split times to be specified for the command are optional (because when removing names you don't have to specify it at all). This means that if any split (other than `finish`) is not specified, they will be defaulted to `0`, i.e it will never ping that split for that runner. If `finish` split is skipped, it will never be written in the splits (as it is optional).
- Eg: `/whitelist add_or_update SathyaPramodh 10 20 30 40 50` would be a valid runner name entry, i.e. all sub `10m` first structure, sub `20m` second structure, sub `30m` blind, sub `40m` eye spy and sub `50m` end enters would show up for that runner.
- `/whitelist add_or_update SathyaPramodh 10 20 30 40 50 60` is also a valid runner name entry, i.e all sub `10m` first structure, sub `20m` second structure, sub `30m` blind, sub `40m` eye spy, sub `50m` end enters and sub `60m` finishes would show up for that runner.
- For public servers (without `#pacemanbot-runner-names-115`), the finish time is capped at `10m`.
- If the finish time is not present for a runner, all finishes would show up.
- Now run `/setup_pb_roles_115` in any channel to setup the valid PB roles to ping for these runners.
- This method of pinging only works rounded to the minute at the moment. So getting pinged for say a Sub 3:30 bastion enter would not be possible with this config.
- This method is useful also when you have a huge number of runners with varied PBs in your server.
- You can even make this channel private but make sure to give the `Read Messages` permission to the `PaceManBot1.15` role for this channel.
- This channel will be used to see what runner's pace-pings need to be sent in this server. So if you want to add more runners, just edit the first message in the channel and add the new runner's in-game name and their splits in a new line.
- This channel is optional however and if it is absent, the bot will check every runner's pace and send them if the conditions are met and the bot will send online pings only (pings only when the runner is live).
- You can even setup a channel named `#pacemanbot-runner-leaderboard-115` to have your own personal leaderboard for your server's whitelisted runners. You need to give perms such as `Read Messages` and `Manage Messages` to the `PaceManBot1.15` role in the same in order for it to be able to send the leaderboard in the first place.
- After you have made the channel, just wait for any whitelisted runner to get a completion. It will update the leaderboard with the name of the runner and the time they got.
- This leaderboard is also sorted automatically as new completions come in!
- Now in any channel (doesn't matter), type in `/setup_roles_115` and the command takes in a couple of required options:
  - `split_name`: This is the name of the split whose roles you want to configure. It can take values like `enter_nether`, `enter_fortress`, `blind`, `eye_spy` and `end_enter`. Any other split name given would just be disregarded.
  - `split_start`: This is the lower bound of the igt in minutes that you want your pace-roles to start from.
  - `split_end`: This is the upper bound of the igt in minutes that you want your pace-roles to end at.
- Eg: If you want all pace-roles for first structure entry from sub 3 minutes all the way to sub 5 minutes setup, then you would type in:
`/setup_roles_115 enter_nether 3 5`. This would create pace-roles for 'Sub 3', 'Sub 3:30', 'Sub 4', 'Sub 4:30' and 'Sub 5'.
- And now in your server's `#roles` channel type in `/send_message_115` to send a message in that channel with drop down boxes that members can choose from the roles that you setup earlier. **NOTE:** If you setup roles again at a later point, you will have to re-send this message.
- And make sure that the bot has the `Send Messages` permission in this channel.
- You can also do `/validate_config_115` to test if all your configuration is setup correctly (very basic checks implemented at the moment). It is recommended to run it each time you change something with the configuration of the server that might affect the bot.
- That's it! You should be getting all pace-pings from paceman.gg on your community discord server while running the tracker! Enjoyy!!

# Migration
Since PaceManBot is now a verified Discord Bot, I have to now also think about how I do things related to user data. Since contents of messages are also sensitive information, I have decided to re-implement how server whitelisting works. This means the old configuration will **STOP** working after we hit 100 servers added. If you added the bot to the server on or before `17th May 2024 11:00 PM IST`, please make sure to run `/migrate` in your server and delete the old configuration message to ensure that the bot works even after we hit 100 servers :)

# Issues
You can report any issues related to the bot [here](https://github.com/paceman-mcsr/pacemanbot/issues).

# Usage (for developers/contributors)
- Build the project with a `.env` file using `cargo build -r` (first compile takes a long time)
- A binary will be created in `target/release/` named `pacemanbot` or `pacemanbot.exe` depending on the OS.
- Run the binary and the bot should start running.

# Thanks
- [Boyenn](https://github.com/dev-boyenn) for the initial implementation of the core with the Websockets backend.
- [Specnr](https://github.com/specnr) for the UI/UX stuff.
- [Nish](https://github.com/ohnishant) for some time formatting help in the role selection message descriptors.
