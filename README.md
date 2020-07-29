# scheduler_bot
A Discord bot for scheduling when people are available.

**WARNING:** This bot is not designed to be run on multiple servers, and most likely has many bugs.

## Description

The bot parses queries into tokens, which can then be processed into the corresponding actions.

A query consists of a single ParamType token, and 0 or more ParamVals
- The ParamType describes the type of query (eg. adding available slots, viewing schedule)
- The ParamVals are the values passed into that type of query, (eg. date range from Mon to Fri)

The complete schedule is split into a per-user schedule, which consists of a single week, split into days and then hours.
- The per-user schedule also contains the user's timezone and preferred name

## Examples
1. `?add from mon to tue 1 2 5 6`: Sets the hours 1, 2, 5, 6 (24h format) from Mon to Tue (inclusive) as available.
2. `?remove weekends from 18 to 23`: Sets the hours from 18 to 23 on weekends (Sat and Sun) as unavailable.
3. `?view`: View your own schedule.
4. `?available mon`: View a per-hour calendar of who is available when on Mon, empty hours are skipped.
5. `?timezone -7`: Sets your timezone to -7:00. -7 == -700.

## TODO
1. Add help function which displays current commands
2. Improve formatting of displayed schedule
3. likely other things...
