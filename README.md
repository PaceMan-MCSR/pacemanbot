# pacemanbot

A Discord bot to query [paceman.gg](https://paceman.gg), ping pace-roles, and assign pace-roles to users.

## Usage (in your own Discord Server)

1. **Add the Bot**
   - Use [this link](https://discord.com/oauth2/authorize?client_id=1385920308145553530) to add the bot to your Discord server.

2. **Restrict Bot Commands to Admins**
   - Go to your server settings and open the `Integrations` tab.
   - Select `PaceManBotAA` and disable the `@everyone` option under `Role & Members`.
   - Add an `admin` role and enable it for PaceManBotAA in this tab.

3. **Create Channels**
   - Create a channel named `#aa` (pace pings will go here).
     - Give the `PaceManBotAA` role permissions to send, read, and mention roles in this channel.
   - Create a channel named `#aa-runner-names`.

4. **Whitelist Runners**
   - Use the following command:
     ```
     /whitelist <action> <ign> [<adventuring_time_hours> <adventuring_time_minutes> <beaconator_hours> <beaconator_minutes> <hdwgh_hours> <hdwgh_minutes> <finish_hours> <finish_minutes>]
     ```
     - `<action>`: `add_or_update` or `remove`
       - `add_or_update`: Adds or updates a runner's splits.
       - `remove`: Removes a runner.
     - All structure/split times are optional (except when removing).
     - Unspecified splits default to `0` (never pings for that split).
     - If `finish_hours` and `finish_minutes` are both skipped, it won't be written in the splits.
     - **Examples:**
       - `/whitelist add_or_update Its_Saanvi 1 10 2 20 3 30`
       - `/whitelist add_or_update Its_Saanvi 1 10 2 20 3 30 4 40 5 50`
     - For public servers (without `#aa-runner-names`), finish time is capped at `3h`.
     - If finish time is not present, all finishes show up.

5. **Setup PB Roles**
   - Run `/setup_pb_roles` in any channel to set up valid PB roles to ping for these runners.
   - Useful for servers with many runners and varied PBs.
   - Channel can be private, but `PaceManBotAA` needs `Read Messages` permission.
   - If the channel is absent, the bot checks every runner's pace and sends online pings only (when runner is live).

6. **Setup Leaderboard (Optional)**
   - Create `#aa-runner-leaderboard` for a personal leaderboard.
   - Give `PaceManBotAA` permissions: `Read Messages` and `Manage Messages`.
   - Leaderboard updates automatically as completions come in.

7. **Configure Pace Roles**
   - In any channel, use:
     ```
     /setup_roles <split_name> <split_start> <split_end>
     ```
     - `split_name`: `adventuring_time`, `beaconator`, `hdwgh`
     - `split_start`: Lower bound of IGT in hours for pace-roles to start from.
     - `split_end`: Upper bound of IGT in hours for pace-roles to end at.
     - **Example:** `/setup_roles adventuring_time 3 5`
       - Sets up pace-roles for 'Sub 3', 'Sub 3:30', 'Sub 4', 'Sub 4:30', 'Sub 5'.

8. **Send Role Selection Message**
   - In your server's `#roles` channel, type `/send_message` to send a message with dropdowns for members to choose roles.
     - **NOTE:** If you set up roles again later, re-send this message.
     - Ensure the bot has `Send Messages` permission.

9. **Validate Configuration**
   - Use `/validate_config` to check if your setup is correct (basic checks only).
   - Recommended after any configuration change.

10. **Additional Notes**
    - Enjoy pace-pings from paceman.gg on your Discord server while running the AA tracker!
   

## Issues

Report issues [here](https://github.com/paceman-mcsr/pacemanbot/issues).


## Contributing
You may look at the contributor docs [here](https://github.com/paceman-mcsr/pacemanbot/blob/main/CONTRIBUTING.md).


## Thanks

- [Boyenn](https://github.com/dev-boyenn) for the initial implementation of the core with the Websockets backend.
- [Specnr](https://github.com/specnr) for the UI/UX stuff.
- [Nish](https://github.com/ohnishant) for time formatting help in the role selection message descriptors.
