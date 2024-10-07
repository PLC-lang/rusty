searchState.loadedDescShard("chrono", 0, "Chrono: Date and Time for Rust\nApril\nAugust\nISO 8601 calendar date with time zone.\nISO 8601 combined date and time with time zone.\nThe common set of methods for date component.\nA duration in calendar days.\nDecember\nAlias of <code>TimeDelta</code>.\nFebruary\nThe time zone with fixed offset, from UTC-23:59:59 to …\nFriday.\nJanuary\nJuly\nJune\nThe local timescale.\nThe maximum possible <code>Date</code>.\nThe maximum possible <code>DateTime&lt;Utc&gt;</code>.\nThe minimum possible <code>Date</code>.\nThe minimum possible <code>DateTime&lt;Utc&gt;</code>.\nMarch\nMay\nMonday.\nThe month of the year.\nA duration in calendar months\nISO 8601 calendar date without timezone. Allows for every …\nISO 8601 combined date and time without timezone.\nISO 8601 time without timezone. Allows for the nanosecond …\nNovember\nOctober\nThe offset from the local time to UTC.\nAn associated offset type. This type is used to store the …\nOut of range error type used in various converting APIs\nSaturday.\nSeptember\nSunday.\nThursday.\nTime duration with nanosecond precision.\nThe time zone.\nThe common set of methods for time component.\nTuesday.\nThe Unix Epoch, 1970-01-01 00:00:00 UTC.\nThe UTC time zone. This is the most efficient time zone …\nWednesday.\nThe day of week.\nReturns the <code>TimeDelta</code> as an absolute (non-negative) value.\nMakes a new <code>DateTime</code> from the current date, hour, minute …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute …\nMakes a new <code>DateTime</code> from the current date and given …\nReturns the total number of months in the <code>Months</code> instance.\nAdd two <code>TimeDelta</code>s, returning <code>None</code> if overflow occurred.\nAdd a duration in <code>Days</code> to the date part of the <code>DateTime</code>.\nAdds given <code>Months</code> to the current date and time.\nAdds given <code>TimeDelta</code> to the current date.\nAdds given <code>TimeDelta</code> to the current date and time.\nDivide a <code>TimeDelta</code> with a i32, returning <code>None</code> if dividing …\nMultiply a <code>TimeDelta</code> with a i32, returning <code>None</code> if …\nSubtract two <code>TimeDelta</code>s, returning <code>None</code> if overflow …\nSubtract a duration in <code>Days</code> from the date part of the …\nSubtracts given <code>Months</code> from the current date and time.\nSubtracts given <code>TimeDelta</code> from the current date.\nSubtracts given <code>TimeDelta</code> from the current date and time.\nRetrieves the date component with an associated timezone.\nRetrieves the date component.\nParses a string with the specified format string and …\nReturns the day of month starting from 1.\nReturns the day of month starting from 0.\nMakes a new <code>TimeDelta</code> with the given number of days.\nThe number of days since the given day.\nReturns the fixed offset from UTC to the local time stored.\nFix the offset from UTC to its current value, dropping the …\nFormat a <code>TimeDelta</code> using the ISO 8601 format\nFormatting (and parsing) utilities for date and time.\nFormats the date with the specified format string. See the …\nFormats the combined date and time per the specified …\nFormats the date with the specified formatting items.\nFormats the combined date and time with the specified …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert this <code>DateTime&lt;Utc&gt;</code> instance into a …\nConvert this <code>DateTime&lt;Utc&gt;</code> instance into a <code>DateTime&lt;Local&gt;</code> …\nConvert this <code>DateTime&lt;Local&gt;</code> instance into a <code>DateTime&lt;Utc&gt;</code> …\nConvert this <code>DateTime&lt;FixedOffset&gt;</code> instance into a …\nConvert this <code>DateTime&lt;FixedOffset&gt;</code> instance into a …\nConvert this <code>DateTime&lt;Local&gt;</code> instance into a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nMakes a new <code>DateTime</code> from a <code>NaiveDateTime</code> in <em>local</em> time …\nConverts the local <code>NaiveDate</code> to the timezone-aware <code>Date</code> if …\nConverts the local <code>NaiveDateTime</code> to the timezone-aware …\nMakes a new <code>DateTime</code> from its components: a <code>NaiveDateTime</code> …\nReconstructs the time zone from the offset.\nCreates a <code>TimeDelta</code> object from <code>std::time::Duration</code>\nMakes a new <code>DateTime&lt;Utc&gt;</code> from the number of non-leap …\nCreates a new <code>DateTime&lt;Utc&gt;</code> from the number of non-leap …\nMakes a new <code>DateTime&lt;Utc&gt;</code> from the number of non-leap …\nCreates a new <code>DateTime&lt;Utc&gt;</code> from the number of non-leap …\nReturns an <code>Option&lt;Month&gt;</code> from a i64, assuming a 1-index, …\nMakes a new <code>Date</code> with given <em>UTC</em> date and offset. The local …\nMakes a new <code>DateTime</code> from its components: a <code>NaiveDateTime</code> …\nConverts the UTC <code>NaiveDate</code> to the local time. The UTC is …\nConverts the UTC <code>NaiveDateTime</code> to the local time. The UTC …\nReturns the hour number from 0 to 23.\nReturns the hour number from 1 to 12 with a boolean flag, …\nReturns the hour number from 1 to 12 with a boolean flag, …\nMakes a new <code>TimeDelta</code> with the given number of hours.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if the <code>TimeDelta</code> equals <code>TimeDelta::zero()</code>.\nReturns the ISO week.\nMakes a new <code>Date</code> from ISO week date (year and week …\nMakes a new <code>Date</code> from ISO week date (year and week …\nThe maximum possible <code>TimeDelta</code>: <code>i64::MAX</code> milliseconds.\nMakes a new <code>TimeDelta</code> with the given number of …\nMakes a new <code>TimeDelta</code> with the given number of …\nThe minimum possible <code>TimeDelta</code>: <code>-i64::MAX</code> milliseconds.\nReturns the minute number from 0 to 59.\nMakes a new <code>TimeDelta</code> with the given number of minutes.\nReturns the month number starting from 1.\nReturns the month number starting from 0.\nDate and time types unconcerned with timezones.\nReturns a view to the naive local date.\nReturns a view to the naive local datetime.\nReturns a view to the naive UTC date.\nReturns a view to the naive UTC datetime.\nGet the name of the month\nReturns the number of nanoseconds since the whole non-leap …\nMakes a new <code>TimeDelta</code> with the given number of nanoseconds.\nMakes a new <code>TimeDelta</code> with given number of seconds and …\nConstruct a new <code>Months</code> from a number of months\nReturns the total number of whole days in the <code>TimeDelta</code>.\nCounts the days in the proleptic Gregorian calendar, with …\nCounts the days in the proleptic Gregorian calendar, with …\nReturns a day-of-week number starting from Monday = 0.\nReturns a day-of-week number starting from Sunday = 0.\nReturns the total number of whole hours in the <code>TimeDelta</code>.\nReturns the total number of whole microseconds in the …\nReturns the total number of whole milliseconds in the …\nReturns the total number of whole minutes in the <code>TimeDelta</code>.\nReturns the total number of whole nanoseconds in the …\nReturns the total number of whole seconds in the <code>TimeDelta</code>.\nReturns the number of non-leap seconds past the last …\nReturns the number of non-leap seconds past the last …\nReturns the total number of whole weeks in the <code>TimeDelta</code>.\nReturns a day-of-week number starting from Monday = 1. …\nReturns a month-of-year number starting from January = 1.\nReturns a day-of-week number starting from Sunday = 1.\nThe time zone, which calculates offsets from the local …\nRetrieves an associated offset from UTC.\nRetrieves an associated offset from UTC.\nCreates the offset(s) for given local <code>NaiveDate</code> if …\nCreates the offset(s) for given local <code>NaiveDateTime</code> if …\nCreates the offset for given UTC <code>NaiveDate</code>. This cannot …\nCreates the offset for given UTC <code>NaiveDateTime</code>. This …\nReturns the day of year starting from 1.\nReturns the day of year starting from 0.\nParses a string from a user-specified format into a …\nParses an RFC 2822 date-and-time string into a …\nParses an RFC 3339 date-and-time string into a …\nParses a string from a user-specified format into a …\nCompare two DateTimes based on their true time, ignoring …\nMakes a new <code>Date</code> for the prior date.\nThe previous day in the week.\nThe previous month.\nMakes a new <code>Date</code> for the prior date.\nA convenience module appropriate for glob imports (…\nFunctionality for rounding or truncating a <code>DateTime</code> by a …\nReturns the second number from 0 to 59.\nMakes a new <code>TimeDelta</code> with the given number of seconds.\nSubtracts another <code>Date</code> from the current date. Returns a …\nSubtracts another <code>DateTime</code> from the current date and time. …\nReturns the number of nanoseconds such that …\nMakes a new <code>Date</code> for the next date.\nThe next day in the week.\nThe next month.\nMakes a new <code>Date</code> for the next date.\nRetrieves the time component.\nMakes a new <code>DateTime</code> from the number of non-leap seconds …\nReturns the number of non-leap seconds since January 1, …\nMakes a new <code>DateTime</code> from the number of non-leap …\nReturns the number of non-leap-microseconds since January …\nMakes a new <code>DateTime</code> from the number of non-leap …\nReturns the number of non-leap-milliseconds since January …\nMakes a new <code>DateTime</code> from the number of non-leap …\nMakes a new <code>DateTime</code> from the number of non-leap …\nReturns the number of non-leap-nanoseconds since January …\nReturns the number of non-leap-nanoseconds since January …\nMakes a new <code>DateTime</code> from the number of non-leap seconds …\nReturns the number of microseconds since the last second …\nReturns the number of milliseconds since the last second …\nReturns the number of nanoseconds since the last second …\nRetrieves an associated time zone.\nRetrieves an associated time zone.\nReturns an RFC 2822 date and time string such as …\nReturns an RFC 3339 and ISO 8601 date and time string such …\nReturn an RFC 3339 and ISO 8601 date and time string with …\nCreates a <code>std::time::Duration</code> object from a <code>TimeDelta</code>.\nTurn this <code>DateTime</code> into a <code>DateTime&lt;Utc&gt;</code>, dropping the …\nMakes a new <code>TimeDelta</code> with the given number of days.\nMakes a new <code>TimeDelta</code> with the given number of hours.\nMakes a new <code>TimeDelta</code> with the given number of …\nMakes a new <code>TimeDelta</code> with the given number of minutes.\nMakes a new <code>TimeDelta</code> with the given number of seconds.\nMakes a new <code>TimeDelta</code> with the given number of weeks.\nReturns the day of week.\nMakes a new <code>TimeDelta</code> with the given number of weeks.\nMakes a new value with the day of month (starting from 1) …\nMakes a new <code>DateTime</code> with the day of month (starting from …\nMakes a new value with the day of month (starting from 0) …\nMakes a new <code>DateTime</code> with the day of month (starting from …\nMakes a new value with the hour number changed.\nMakes a new <code>DateTime</code> with the hour number changed.\nMakes a new value with the minute number changed.\nMakes a new <code>DateTime</code> with the minute number changed.\nMakes a new value with the month number (starting from 1) …\nMakes a new <code>DateTime</code> with the month number (starting from …\nMakes a new value with the month number (starting from 0) …\nMakes a new <code>DateTime</code> with the month number (starting from …\nMakes a new value with nanoseconds since the whole …\nMakes a new <code>DateTime</code> with nanoseconds since the whole …\nMakes a new value with the day of year (starting from 1) …\nMakes a new <code>DateTime</code> with the day of year (starting from …\nMakes a new value with the day of year (starting from 0) …\nMakes a new <code>DateTime</code> with the day of year (starting from …\nMakes a new value with the second number changed.\nMakes a new <code>DateTime</code> with the second number changed.\nSet the time to a new fixed time on the existing date.\nChanges the associated time zone. This does not change the …\nChanges the associated time zone. The returned <code>DateTime</code> …\nMakes a new value with the year number changed, while …\nMakes a new <code>DateTime</code> with the year number changed, while …\nMake a new <code>DateTime</code> from year, month, day, time components …\nReturns the year number in the calendar date.\nReturns the absolute year number starting from 1 with a …\nReturns the absolute year number starting from 1 with a …\nReturns the number of whole years from the given <code>base</code> …\nRetrieve the elapsed years from now to the given <code>DateTime</code>.\nMakes a new <code>Date</code> from year, month, day and the current …\nMakes a new <code>Date</code> from year, month, day and the current …\nMakes a new <code>Date</code> from year, day of year (DOY or “ordinal…\nMakes a new <code>Date</code> from year, day of year (DOY or “ordinal…\nA <code>TimeDelta</code> where the stored seconds and nanoseconds are …\nAutomatically select one of <code>Secs</code>, <code>Millis</code>, <code>Micros</code>, or <code>Nanos</code> …\nThere was an error on the formatting string, or there were …\nColon (<code>:</code>) as separator\nThe separator between hours and minutes in an offset.\nDay of the month (FW=PW=2).\nA <em>temporary</em> object which can be used as an argument to …\nContains the error value\nIssues a formatting error. Used to signal an invalid …\nFixed-format item types.\nFixed-format item.\nHour number in the 24-hour clocks (FW=PW=2).\nHour number in the 12-hour clocks (FW=PW=2).\nFormat offset from UTC as only hours. Not recommended, it …\nThere is no possible date and time value with given set of …\nInternal uses only.\nInternal uses only.\nAn opaque type representing fixed-format item types for …\nAn opaque type representing numeric item types for …\nThe input string has some invalid character sequence for …\nWeek number in the ISO week date (FW=PW=2).\nYear in the ISO week date (FW=4, PW=∞). May accept years …\nYear in the ISO week date, divided by 100 (FW=PW=2). …\nYear in the ISO week date, modulo 100 (FW=PW=2). Cannot be …\nA single formatting item. This is used for both formatting …\nA literally printed and parsed text.\nFull month names.\nFull day of the week names.\nAM/PM.\nNo separator when formatting, colon allowed when parsing.\nUse fixed 6 subsecond digits. This corresponds to …\nUse fixed 3 subsecond digits. This corresponds to …\nThe number of minutes since the last whole hour (FW=PW=2).\nFormat offset from UTC as hours and minutes. Any seconds …\nMonth (FW=PW=2).\nUse fixed 9 subsecond digits. This corresponds to …\nThe number of nanoseconds since the last whole second …\nAn optional dot plus one or more digits for left-aligned …\nSame as <code>Nanosecond</code> but the accuracy is fixed to 3.\nSame as <code>Nanosecond</code> but the accuracy is fixed to 6.\nSame as <code>Nanosecond</code> but the accuracy is fixed to 9.\nNo padding.\nNo separator\nGiven set of fields is not enough to make a requested date …\nDay of the week, where Sunday = 0 and Saturday = 6 …\nNumeric item types. They have associated formatting width …\nNumeric item. Can be optionally padded to the maximal …\nType for specifying the format of UTC offsets.\nThe precision of an offset from UTC formatting item.\nContains the success value\nFormat offset from UTC as hours, and optionally with …\nFormat offset from UTC as hours and optionally minutes and …\nFormat offset from UTC as hours and minutes, and …\nDay of the year (FW=PW=3).\nGiven field is out of permitted range.\nSame as <code>Literal</code> but with the string owned by the item.\nSame as <code>Space</code> but with the string owned by the item.\nPadding characters for numeric items.\nAn error from the <code>parse</code> function.\nThe category of parse error\nSame as <code>Result&lt;T, ParseError&gt;</code>.\nA type to hold parsed fields of date and time that can …\nRFC 2822 date and time syntax. Commonly used for email and …\nRFC 3339 &amp; ISO 8601 date and time syntax.\nThe number of seconds since the last whole minute …\nFormat offset from UTC as hours, minutes and seconds.\nSpecific formatting options for seconds. This may be …\nFormat whole seconds only, with no decimal point nor …\nAbbreviated month names.\nAbbreviated day of the week names.\nSpace padding.\nWhitespace. Prints literally but reads zero or more …\nThe number of non-leap seconds since the midnight UTC on …\nTimezone name.\nSame as <code>TimezoneOffsetColon</code> but prints no colon. Parsing …\nOffset from the local time to UTC (<code>+09:00</code> or <code>-04:00</code> or …\nOffset from the local time to UTC (<code>+09:00</code> or <code>-04:00</code> or <code>Z</code>).\nOffset from the local time to UTC with seconds (<code>+09:00:00</code> …\nOffset from the local time to UTC without minutes (<code>+09</code> or …\nSame as <code>TimezoneOffsetColonZ</code> but prints no colon. Parsing …\nAll formatting items have been read but there is a …\nThe input string has been prematurely ended.\nAM/PM.\nWeek number, where the week 1 starts at the first Monday …\nWeek number, where the week 1 starts at the first Sunday …\nDay of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).\nFull Gregorian year (FW=4, PW=∞). May accept years …\nGregorian year divided by 100 (century number; FW=PW=2). …\nGregorian year modulo 100 (FW=PW=2). Cannot be negative.\nZero (<code>0</code>) padding.\nRepresent <code>+00:00</code> as <code>Z</code>.\nSeparator between hours, minutes and seconds.\nGet the <code>day</code> of the month field if set.\nTries to format given arguments with given formatting …\nFormats single formatting item\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the <code>hour_div_12</code> field (am/pm) if set.\nGet the <code>hour_mod_12</code> field if set.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet the <code>isoweek</code> field that is part of an ISO 8601 week date…\nGet the <code>isoyear</code> field that is part of an ISO 8601 week date…\nGet the <code>isoyear_div_100</code> field that is part of an ISO 8601 …\nGet the <code>isoyear_mod_100</code> field that is part of an ISO 8601 …\nThe category of parse error\nGet the <code>minute</code> field if set.\nGet the <code>month</code> field if set.\nGet the <code>nanosecond</code> field if set.\nMakes a new <code>DelayedFormat</code> value out of local date and time.\nReturns the initial value of parsed parts.\nMakes a new <code>DelayedFormat</code> value out of local date and time …\nGet the <code>offset</code> field if set.\nGet the <code>ordinal</code> (day of the year) field if set.\nPad the hour value to two digits.\nTries to parse given string into <code>parsed</code> with given …\nTries to parse given string into <code>parsed</code> with given …\nSee <code>OffsetPrecision</code>.\nGet the <code>second</code> field if set.\nSet the <code>hour_div_12</code> am/pm field to the given value.\nSet the <code>day</code> of the month field to the given value.\nSet the <code>hour_div_12</code> and <code>hour_mod_12</code> fields to the given …\nSet the <code>hour_mod_12</code> field, for the hour number in 12-hour …\nSet the <code>isoweek</code> field for an ISO 8601 week date to the …\nSet the <code>isoyear</code> field, that is part of an ISO 8601 week …\nSet the <code>isoyear_div_100</code> field, that is part of an ISO 8601 …\nSet the <code>isoyear_mod_100</code> field, that is part of an ISO 8601 …\nSet the <code>minute</code> field to the given value.\nSet the <code>month</code> field to the given value.\nSet the <code>nanosecond</code> field to the given value.\nSet the <code>offset</code> field to the given value.\nSet the <code>ordinal</code> (day of the year) field to the given value.\nSet the <code>second</code> field to the given value.\nSet the <code>timestamp</code> field to the given value.\nSet the <code>week_from_mon</code> week number field to the given value.\nSet the <code>week_from_sun</code> week number field to the given value.\nSet the <code>weekday</code> field to the given value.\nSet the <code>year</code> field to the given value.\nSet the <code>year_div_100</code> field to the given value.\nSet the <code>year_mod_100</code> field to the given value.\n<code>strftime</code>/<code>strptime</code>-inspired date and time formatting syntax.\nGet the <code>timestamp</code> field if set.\nReturns a parsed timezone-aware date and time out of given …\nReturns a parsed timezone-aware date and time out of given …\nReturns a parsed fixed time zone offset out of given …\nReturns a parsed naive date out of given fields.\nReturns a parsed naive date and time out of given fields, …\nReturns a parsed naive time out of given fields.\nConvert items that contain a reference to the format …\nGet the <code>week_from_mon</code> field if set.\nGet the <code>week_from_sun</code> field if set.\nGet the <code>weekday</code> field if set.\nGet the <code>year</code> field if set.\nGet the <code>year_div_100</code> field if set.\nGet the <code>year_mod_100</code> field if set.\nParsing iterator for <code>strftime</code>-like format strings.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new parsing iterator from a <code>strftime</code>-like format …\nParse format string into a <code>Vec</code> of formatting <code>Item</code>’s.\nParse format string into a <code>Vec</code> of <code>Item</code>’s that contain no …\nA duration in calendar days.\nISO 8601 week.\nThe maximum possible <code>NaiveDate</code> (December 31, 262142 CE).\nThe maximum possible <code>NaiveDateTime</code>.\nThe maximum possible <code>NaiveDate</code> (December 31, 262143 CE).\nThe maximum possible <code>NaiveDateTime</code>.\nThe minimum possible <code>NaiveDate</code> (January 1, 262144 BCE).\nThe minimum possible <code>NaiveDateTime</code>.\nThe earliest possible <code>NaiveTime</code>\nThe minimum possible <code>NaiveDate</code> (January 1, 262145 BCE).\nThe minimum possible <code>NaiveDateTime</code>.\nISO 8601 calendar date without timezone. Allows for every …\nIterator over <code>NaiveDate</code> with a step size of one day.\nISO 8601 combined date and time without timezone.\nIterator over <code>NaiveDate</code> with a step size of one week.\nISO 8601 time without timezone. Allows for the nanosecond …\nA week represented by a <code>NaiveDate</code> and a <code>Weekday</code> which is …\nThe Unix Epoch, 1970-01-01 00:00:00.\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nMakes a new <code>NaiveDateTime</code> from the current date, hour, …\nConverts the <code>NaiveDateTime</code> into a timezone-aware …\nMakes a new <code>NaiveDateTime</code> from the current date and given …\nConverts the <code>NaiveDateTime</code> into the timezone-aware …\nAdd a duration in <code>Days</code> to the date\nAdd a duration in <code>Days</code> to the date part of the …\nAdd a duration in <code>Months</code> to the date\nAdds given <code>Months</code> to the current date and time.\nAdds given <code>FixedOffset</code> to the current datetime. Returns …\nAdds the number of whole days in the given <code>TimeDelta</code> to …\nAdds given <code>TimeDelta</code> to the current date and time.\nSubtract a duration in <code>Days</code> from the date\nSubtract a duration in <code>Days</code> from the date part of the …\nSubtract a duration in <code>Months</code> from the date\nSubtracts given <code>Months</code> from the current date and time.\nSubtracts given <code>FixedOffset</code> from the current datetime. …\nSubtracts the number of whole days in the given <code>TimeDelta</code> …\nSubtracts given <code>TimeDelta</code> from the current date and time.\nRetrieves a date component.\nReturns the day of month starting from 1.\nReturns the day of month starting from 1.\nReturns the day of month starting from 0.\nReturns the day of month starting from 0.\nReturns a <code>RangeInclusive&lt;T&gt;</code> representing the whole week …\nReturns a date representing the first day of the week.\nFormats the date with the specified format string. See the …\nFormats the combined date and time with the specified …\nFormats the time with the specified format string. See the …\nFormats the date with the specified formatting items. …\nFormats the combined date and time with the specified …\nFormats the time with the specified formatting items. …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a <code>NaiveDate</code> to a <code>NaiveDateTime</code> of the same date …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nMakes a new <code>NaiveTime</code> from hour, minute and second.\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute, second and …\nMakes a new <code>NaiveTime</code> from hour, minute and second.\nMakes a new <code>NaiveDate</code> from the ISO week date (year, week …\nMakes a new <code>NaiveDate</code> from the ISO week date (year, week …\nMakes a new <code>NaiveDate</code> from a day’s number in the …\nMakes a new <code>NaiveDate</code> from a day’s number in the …\nMakes a new <code>NaiveTime</code> from the number of seconds since …\nMakes a new <code>NaiveTime</code> from the number of seconds since …\nMakes a new <code>NaiveDateTime</code> corresponding to a UTC date and …\nCreates a new NaiveDateTime from microseconds since the …\nCreates a new NaiveDateTime from milliseconds since the …\nCreates a new NaiveDateTime from nanoseconds since the …\nMakes a new <code>NaiveDateTime</code> corresponding to a UTC date and …\nMakes a new <code>NaiveDate</code> by counting the number of …\nMakes a new <code>NaiveDate</code> by counting the number of …\nMakes a new <code>NaiveDate</code> from the calendar date (year, month …\nMakes a new <code>NaiveDate</code> from the calendar date (year, month …\nMakes a new <code>NaiveDate</code> from the ordinal date (year and day …\nMakes a new <code>NaiveDate</code> from the ordinal date (year and day …\nReturns the hour number from 0 to 23.\nReturns the hour number from 0 to 23.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns an iterator that steps by days across all …\nReturns an iterator that steps by weeks across all …\nReturns a date representing the last day of the week.\nReturns <code>true</code> if this is a leap year.\nReturns the minute number from 0 to 59.\nReturns the minute number from 0 to 59.\nReturns the month number starting from 1.\nReturns the month number starting from 1.\nReturns the month number starting from 0.\nReturns the month number starting from 0.\nReturns the number of nanoseconds since the whole non-leap …\nReturns the number of nanoseconds since the whole non-leap …\nMakes a new <code>NaiveDateTime</code> from date and time components. …\nConstruct a new <code>Days</code> from a number of days\nReturns the number of non-leap seconds past the last …\nReturns the day of year starting from 1.\nReturns the day of year starting from 1.\nReturns the day of year starting from 0.\nReturns the day of year starting from 0.\nAdds given <code>TimeDelta</code> to the current time, and also returns …\nSubtracts given <code>TimeDelta</code> from the current time, and also …\nParses a string from a user-specified format into a new …\nParses a string with the specified format string and …\nParses a string from a user-specified format into a new …\nParses a string with the specified format string and …\nParses a string with the specified format string and …\nParses a string with the specified format string and …\nMakes a new <code>NaiveDate</code> for the previous calendar date.\nMakes a new <code>NaiveDate</code> for the previous calendar date.\nReturns the second number from 0 to 59.\nReturns the second number from 0 to 59.\nSubtracts another <code>NaiveDate</code> from the current date. Returns …\nSubtracts another <code>NaiveDateTime</code> from the current date and …\nSubtracts another <code>NaiveTime</code> from the current time. Returns …\nMakes a new <code>NaiveDate</code> for the next calendar date.\nMakes a new <code>NaiveDate</code> for the next calendar date.\nRetrieves a time component.\nReturns the number of non-leap seconds since the midnight …\nReturns the number of non-leap <em>microseconds</em> since midnight …\nReturns the number of non-leap <em>milliseconds</em> since midnight …\nReturns the number of non-leap <em>nanoseconds</em> since midnight …\nReturns the number of non-leap <em>nanoseconds</em> since midnight …\nReturns the number of microseconds since the last whole …\nReturns the number of milliseconds since the last whole …\nReturns the number of nanoseconds since the last whole …\nReturns the <code>NaiveWeek</code> that the date belongs to, starting …\nReturns the ISO week number starting from 1.\nReturns the ISO week number starting from 0.\nReturns the day of week.\nReturns the day of week.\nMakes a new <code>NaiveDate</code> with the day of month (starting from …\nMakes a new <code>NaiveDateTime</code> with the day of month (starting …\nMakes a new <code>NaiveDate</code> with the day of month (starting from …\nMakes a new <code>NaiveDateTime</code> with the day of month (starting …\nMakes a new <code>NaiveDateTime</code> with the hour number changed.\nMakes a new <code>NaiveTime</code> with the hour number changed.\nMakes a new <code>NaiveDateTime</code> with the minute number changed.\nMakes a new <code>NaiveTime</code> with the minute number changed.\nMakes a new <code>NaiveDate</code> with the month number (starting from …\nMakes a new <code>NaiveDateTime</code> with the month number (starting …\nMakes a new <code>NaiveDate</code> with the month number (starting from …\nMakes a new <code>NaiveDateTime</code> with the month number (starting …\nMakes a new <code>NaiveDateTime</code> with nanoseconds since the whole …\nMakes a new <code>NaiveTime</code> with nanoseconds since the whole …\nMakes a new <code>NaiveDate</code> with the day of year (starting from …\nMakes a new <code>NaiveDateTime</code> with the day of year (starting …\nMakes a new <code>NaiveDate</code> with the day of year (starting from …\nMakes a new <code>NaiveDateTime</code> with the day of year (starting …\nMakes a new <code>NaiveDateTime</code> with the second number changed.\nMakes a new <code>NaiveTime</code> with the second number changed.\nMakes a new <code>NaiveDate</code> with the year number changed, while …\nMakes a new <code>NaiveDateTime</code> with the year number changed, …\nReturns the year number in the calendar date.\nReturns the year number in the calendar date.\nReturns the year number for this ISO week.\nReturns the number of whole years from the given <code>base</code> …\nThe local time is <em>ambiguous</em> because there is a <em>fold</em> in the …\nThe local time is <em>ambiguous</em> because there is a <em>fold</em> in the …\nThe time zone with fixed offset, from UTC-23:59:59 to …\nThe local timescale.\nOld name of <code>MappedLocalTime</code>. See that type for more …\nThe result of mapping a local time to a concrete instant …\nThe local time does not exist because there is a <em>gap</em> in …\nThe local time does not exist because there is a <em>gap</em> in …\nThe offset from the local time to UTC.\nAn associated offset type. This type is used to store the …\nThe local time maps to a single unique result.\nThe local time maps to a single unique result.\nThe time zone.\nThe UTC time zone. This is the most efficient time zone …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute, …\nMakes a new <code>DateTime</code> from the current date, hour, minute …\nMakes a new <code>DateTime</code> from the current date, hour, minute …\nMakes a new <code>DateTime</code> from the current date and given …\nMakes a new <code>DateTime</code> from the current date and given …\nParses a string with the specified format string and …\nReturns the earliest possible result of a the time zone …\nReturns the earliest possible result of a the time zone …\nMakes a new <code>FixedOffset</code> for the Eastern Hemisphere with …\nMakes a new <code>FixedOffset</code> for the Eastern Hemisphere with …\nReturns the fixed offset from UTC to the local time stored.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts the local <code>NaiveDate</code> to the timezone-aware <code>Date</code> if …\nConverts the local <code>NaiveDateTime</code> to the timezone-aware …\nReconstructs the time zone from the offset.\nConverts the UTC <code>NaiveDate</code> to the local time. The UTC is …\nConverts the UTC <code>NaiveDateTime</code> to the local time. The UTC …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMakes a new <code>Date</code> from ISO week date (year and week …\nMakes a new <code>Date</code> from ISO week date (year and week …\nReturns the latest possible result of a the time zone …\nReturns the latest possible result of a the time zone …\nReturns the number of seconds to add to convert from UTC …\nMaps a <code>MappedLocalTime&lt;T&gt;</code> into <code>MappedLocalTime&lt;U&gt;</code> with …\nMaps a <code>MappedLocalTime&lt;T&gt;</code> into <code>MappedLocalTime&lt;U&gt;</code> with …\nReturns a <code>DateTime&lt;Local&gt;</code> which corresponds to the current …\nReturns a <code>DateTime&lt;Utc&gt;</code> which corresponds to the current …\nCreates the offset(s) for given local <code>NaiveDate</code> if …\nCreates the offset(s) for given local <code>NaiveDateTime</code> if …\nCreates the offset for given UTC <code>NaiveDate</code>. This cannot …\nCreates the offset for given UTC <code>NaiveDateTime</code>. This …\nReturns <code>Some</code> if the time zone mapping has a single result.\nReturns <code>Some</code> if the time zone mapping has a single result.\nMakes a new <code>DateTime</code> from the number of non-leap seconds …\nMakes a new <code>DateTime</code> from the number of non-leap …\nMakes a new <code>DateTime</code> from the number of non-leap …\nMakes a new <code>DateTime</code> from the number of non-leap …\nMakes a new <code>DateTime</code> from the number of non-leap …\nMakes a new <code>DateTime</code> from the number of non-leap seconds …\nReturns a <code>Date</code> which corresponds to the current date.\nReturns a <code>Date</code> which corresponds to the current date.\nReturns a single unique conversion result or panics.\nReturns a single unique conversion result or panics.\nReturns the number of seconds to add to convert from the …\nMakes a new <code>FixedOffset</code> for the Western Hemisphere with …\nMakes a new <code>FixedOffset</code> for the Western Hemisphere with …\nMake a new <code>DateTime</code> from year, month, day, time components …\nMakes a new <code>Date</code> from year, month, day and the current …\nMakes a new <code>Date</code> from year, month, day and the current …\nMakes a new <code>Date</code> from year, day of year (DOY or “ordinal…\nMakes a new <code>Date</code> from year, day of year (DOY or “ordinal…\nError when <code>TimeDelta.num_nanoseconds</code> exceeds the limit.\nError when the TimeDelta exceeds the TimeDelta from or …\nExtension trait for rounding or truncating a DateTime by a …\nError that can occur in rounding or truncating\nAn error from rounding by <code>TimeDelta</code>\nExtension trait for subsecond rounding or truncation to a …\nError when <code>DateTime.timestamp_nanos</code> exceeds the limit.\nReturn a copy rounded by TimeDelta.\nReturn a copy truncated by TimeDelta.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturn a copy rounded to the specified number of subsecond …\nReturn a copy truncated to the specified number of …")