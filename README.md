# pacemanbot

A Discord bot to query [paceman.gg](https://paceman.gg), ping pace-roles, and assign pace-roles to users.

## Usage (in your own Discord Server)

1. **Add the Bot**
   - Use [this link](https://discord.com/api/oauth2/authorize?client_id=1136700221603192873&permissions=2416126992&scope=bot%20applications.commands) to add the bot to your Discord server.

2. **Restrict Bot Commands to Admins**
   - Go to your server settings and open the `Integrations` tab.
   - Select `PaceManBot` and disable the `@everyone` option under `Role & Members`.
   - Add an `admin` role and enable it for PaceManBot in this tab.

3. **Create Channels**
   - Create a channel named `#pacemanbot` (pace pings will go here).
     - Give the `PaceManBot` role permissions to send, read, and mention roles in this channel.
   - Create a channel named `#pacemanbot-runner-names`.

4. **Whitelist Runners**
   - Use the following command:
     ```
     /whitelist <action> <ign> [<first_structure> <second_structure> <blind> <eye_spy> <end_enter> <finish>]
     ```
     - `<action>`: `add_or_update` or `remove`
       - `add_or_update`: Adds or updates a runner's splits.
       - `remove`: Removes a runner.
     - All structure/split times are optional (except when removing).
     - Unspecified splits default to `0` (never pings for that split).
     - If `finish` is skipped, it won't be written in the splits.
     - **Examples:**
       - `/whitelist add_or_update Its_Saanvi 10 20 30 40 50`
       - `/whitelist add_or_update Its_Saanvi 10 20 30 40 50 60`
     - For public servers (without `#pacemanbot-runner-names`), finish time is capped at `10m`.
     - If finish time is not present, all finishes show up.

5. **Setup PB Roles**
   - Run `/setup_pb_roles` in any channel to set up valid PB roles to ping for these runners.
   - Pinging works rounded to the minute (e.g., sub 3:30 not possible).
   - Useful for servers with many runners and varied PBs.
   - Channel can be private, but `PaceManBot` needs `Read Messages` permission.
   - If the channel is absent, the bot checks every runner's pace and sends online pings only (when runner is live).

6. **Setup Leaderboard (Optional)**
   - Create `#pacemanbot-runner-leaderboard` for a personal leaderboard.
   - Give `PaceManBot` permissions: `Read Messages` and `Manage Messages`.
   - Leaderboard updates automatically as completions come in.

7. **Configure Pace Roles**
   - In any channel, use:
     ```
     /setup_roles <split_name> <split_start> <split_end>
     ```
     - `split_name`: `first_structure`, `second_structure`, `blind`, `eye_spy`, `end_enter`
     - **Example:** `/setup_roles first_structure 3 5`
       - Sets up pace-roles for 'Sub 3', 'Sub 3:30', 'Sub 4', 'Sub 4:30', 'Sub 5'.
   - Use `/setup_default_roles` to set up all pace-roles for a typical sub 10 pace.

8. **Send Role Selection Message**
   - In your server's `#roles` channel, type `/send_message` to send a message with dropdowns for members to choose roles.
     - **NOTE:** If you set up roles again later, re-send this message.
     - Ensure the bot has `Send Messages` permission.

9. **Validate Configuration**
   - Use `/validate_config` to check if your setup is correct (basic checks only).
   - Recommended after any configuration change.

10. **Additional Notes**
    - Pace-roles for first structure are optional.
    - If not set up, no dropdown for first structure in `/send_message`.
    - Enjoy pace-pings from paceman.gg on your Discord server!

## Contributing
You may look at the contributor docs [here](https://github.com/paceman-mcsr/pacemanbot/blob/main/CONTRIBUTING.md).

## Thanks

- [Boyenn](https://github.com/dev-boyenn) for the initial implementation of the core with the Websockets backend.
- [Specnr](https://github.com/specnr) for the UI/UX stuff.
- [Nish](https://github.com/ohnishant) for time formatting help in the role selection message descriptors.
