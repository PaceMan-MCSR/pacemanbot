# pacemanbot
A Discord bot to query paceman.gg, ping pace-roles and assign pace-roles to users.

# Usage (in your own Discord Server)
- First, use [this link](https://discord.com/api/oauth2/authorize?client_id=1136700221603192873&permissions=2416126992&scope=bot%20applications.commands) to add the bot to your discord server.
- Before we do anything, we would want to disable everyone from accessing the bot commands as they are meant for admins.
- Go to your server settings and go into the `Integrations` tab. Under there, select `PaceManBot` and just disable the option that says `@everyone` under `Role & Members`. You can add in like an `admin` role and enable that role to be able to use this role in this same tab.
- Now in your discord server, create a new channel named `#pacemanbot`.
- This is where your pace pings will go. So make sure to give the role `PaceManBot` all the necessary perms for sending, reading and mentioning roles in that channel.
- And then, create a new channel named `#pacemanbot-runner-names` and send a message in that channel with your in-game name. You can even make this channel private but make sure to give the `Read Messages` permission to the `PaceManBot` role for this channel.
- This channel will be used to see what runner's pace-pings need to be sent in this server. So if you want to add more runners, just edit the first message in the channel and add the new runner's in-game name in a new line.
- This channel is optional however and if it is absent, the bot will check every runner's pace and send them if the conditions are met.
- Now in any channel (doesn't matter), type in `/setup_roles` and the command takes in a couple of required options:
  - `split_name`: This is the name of the split whose roles you want to configure. It can take values like `first_structure`, `second_structure`, `blind`, `eye_spy` and `end_enter`. Any other split name given would just be disregarded.
  - `split_start`: This is the lower bound of the igt in minutes that you want your pace-roles to start from.
  - `split_end`: This is the upper bound of the igt in minutes that you want your pace-roles to end at.
- Eg: If you want all pace-roles for first structure entry from sub 3 minutes all the way to sub 5 minutes setup, then you would type in:
`/setup_roles first_structure 3 5`. This would create pace-roles for 'Sub 3', 'Sub 3:30', 'Sub 4', 'Sub 4:30' and 'Sub 5'.
- You can even setup all pace-roles for a typical sub 10 pace using the `/setup_default_roles` command in any channel.
- And now in your server's `#roles` channel type in `/send_message` to send a message in that channel with drop down boxes that members can choose from the roles that you setup earlier. **NOTE:** If you setup roles again at a later point, you will have to re-send this message.
- And make sure that the bot has the `Send Messages` permission in this channel.
- **NOTE:** The pace-roles for first structure entry is optional. If you don't have any roles setup for first structure, the bot will not send a drop-down for the same when you issue `/send_message`.
- That's it! You should be getting all pace-pings from paceman.gg on your community discord server while running the tracker! Enjoyy!!

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
