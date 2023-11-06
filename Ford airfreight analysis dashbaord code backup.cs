

AOD_Bid_Stats_Merge_2023
The m-code used to recreate Steves report in which he monitors the airfreight wins and losses for the import business


let
    Source = Excel.Workbook(Web.Contents("https://dsvcorp.sharepoint.com/teams/FORD-GlobalAirfreightAward/Shared%20Documents/General/Performance%20Metrics/AOD%20Bidding%20Stats/AOD%20Bid%20Stats%20Merge%202023.xlsx"), null, true),
    Custom1 = Source{[Item="Sheet1",Kind="Sheet"]}[Data],
    #"Promoted Headers" = Table.PromoteHeaders(Custom1, [PromoteAllScalars=true]),
    #"Remove pre-calc_columns" = Table.RemoveColumns(#"Promoted Headers",{"Year", "Month", "Week #", "OD Lane Pairs", "Origin", "Destination", "Response (Y/N)", "Declined", "Award Confirmed", "Response Time", "Time Posted", "After Hours", "Posted During Operating Hours", "Bid During Operating Hours", "Weight (kgs)", "Cost Per KG", "Missed Kgs", "Estimated Revenue"}),
    #"Changed Type" = Table.TransformColumnTypes(#"Remove pre-calc_columns",{{"Shipment", Int64.Type}, {"Call Date", type date}, {"Bill To", type text}, {"Shipper", type text}, {"Ready", type datetime}, {"Consignee", type text}, {"Need", type datetime}, {"Cont", Int64.Type}, {"Pounds", type number}, {"Miles", type number}, {"Response", type any}, {"Who", type text}, {"Award", type text}, {"Unit", type text}, {"Mode", type text}, {"Posted", type datetime}, {"Responded", type datetime}, {"Awarded", type datetime}}),
    #"Extract Year" = Table.AddColumn(#"Changed Type", "Year", each Date.Year([Posted])),
    #"Extract Month" = Table.AddColumn(#"Extract Year", "Month", each Date.MonthName([Posted])),
    #"Extract Week" = Table.AddColumn(#"Extract Month", "Week #", each Date.WeekOfYear([Posted])),
    #"Left join shippers origin" = Table.NestedJoin(#"Extract Week", {"Shipper"}, Import_Shippers, {"Shipper"}, "Import_Shippers", JoinKind.LeftOuter),
    #"Expanded Origin" = Table.ExpandTableColumn(#"Left join shippers origin", "Import_Shippers", {"Country"}, {"Origin.Country"}),
    #"Left join consignee destination" = Table.NestedJoin(#"Expanded Origin", {"Consignee"}, Import_Shippers, {"Consignee"}, "Import_Shippers", JoinKind.LeftOuter),
    #"Expanded Destination" = Table.ExpandTableColumn(#"Left join consignee destination", "Import_Shippers", {"Country_1"}, {"Destination.Country_1"}),
    #"Merged OD Lane Pair" = Table.CombineColumns(#"Expanded Destination",{"Origin.Country", "Destination.Country_1"},Combiner.CombineTextByDelimiter("-", QuoteStyle.None),"OD Lane Pair"),
    #"Added Response Y/N" = Table.AddColumn(#"Merged OD Lane Pair", "Response Y/N", each Value.Is([Response],type text)),
    #"Added Declined" = Table.AddColumn(#"Added Response Y/N", "Declined", each [Response] = "Declined"),
    #"Added Award Confirmed" = Table.AddColumn(#"Added Declined", "Award Confirmed", each [Award] = "Yes"),
    #"Added Response Time" = Table.AddColumn(#"Added Award Confirmed", "Response Time", each if [Response] = "No Response" then "NO RESPONSE" else [Responded]-[Posted]),
    #"Added Time Posted" = Table.AddColumn(#"Added Response Time", "Time Posted", each DateTime.Time([Posted])),
    #"Added After Hours" = Table.AddColumn(#"Added Time Posted", "After Hours", each [Time Posted] >= #time(19, 0, 0)),
    #"Added Weight (kgs)" = Table.AddColumn(#"Added After Hours", "Added Weight (kgs)", each [Pounds] * 0.453592),
    #"Added Missed Kgs" = Table.AddColumn(#"Added Weight (kgs)", "Missed Kgs", each if [Response Time] <> null then if [Response Time]="NO RESPONSE" then [#"Added Weight (kgs)"] else 0 else 0, Int64.Type),
    #"Added Posted During Operating Hours" = Table.AddColumn(#"Added Missed Kgs", "Posted During Operating Hours", each if Time.From([Posted]) >= Time.From(#datetime(1899, 12, 30,6, 30, 0)) and Time.From([Posted]) <= Time.From(#datetime(1899, 12, 30,17, 00, 0)) then true else false),
    #"Added Bid During Operating Hours" = Table.AddColumn(#"Added Posted During Operating Hours", "Bid During Operating Hours", each if [Responded] <> null then if Time.From([Responded]) >= Time.From(#datetime(1899, 12, 30,6, 30, 0)) 
and Time.From([Responded]) <= Time.From(#datetime(1899, 12, 30,17, 00, 0)) then true else false else "BLANK"),
    #"Added Cost per KG" = Table.AddColumn(#"Added Bid During Operating Hours", "Cost per KG", each if [Pounds] <= 100 then 0 else if [Response] = "No Response" then 0 else if [Response] = "Declined" then 0 else [Response] / ([Pounds]*0.453592)),
    #"Created column list" = Table.Column(#"Added Cost per KG", "Cost per KG"),
    #"Removed 0's from list" = List.Select(#"Created column list", each _ <> 0),
    AverageWithoutZeros_value = List.Average(#"Removed 0's from list"),
    #"Estimated Revenue" = Table.AddColumn(#"Added Cost per KG", "newstep", each if [Cost per KG] = 0 then AverageWithoutZeros_value * [Missed Kgs] else [Response]),
    #"Changed Type Cost per KG" = Table.TransformColumnTypes(#"Added Cost per KG",{{"Cost per KG", type number}}),
    #"Grouped Rows" = Table.Group(#"Changed Type Cost per KG", {"Shipment"}, {{"max_posted_date", each List.Max([Posted]), type datetime}, {"all rows", each _, type table [Shipment=nullable number, Call Date=nullable date, Bill To=nullable text, Shipper=nullable text, Ready=nullable datetime, Consignee=nullable text, Need=nullable datetime, Cont=nullable number, Pounds=nullable number, Miles=nullable number, Response=any, Who=nullable text, Award=nullable text, Unit=nullable text, Mode=nullable text, Posted=any, Responded=nullable datetime, Awarded=nullable datetime, Year=nullable number, Month=nullable text, #"Week #"=nullable number, OD Lane Pairs=nullable text, Origin=nullable text, Destination=nullable text, #"Response (Y/N)"=nullable number, Declined=nullable number, Award Confirmed=nullable number, Response Time=any, Time Posted=nullable datetime, After Hours=nullable number, Posted During Operating Hours=nullable logical, Bid During Operating Hours=nullable logical, #"Weight (kgs)"=nullable number, Cost Per KG=any, Missed Kgs=nullable number, Estimated Revenue=nullable number]}}),
    #"Sorted Rows" = Table.Sort(#"Grouped Rows",{{"max_posted_date", Order.Descending}}),
    #"Removed Duplicates" = Table.Distinct(#"Sorted Rows", {"Shipment"}),
    #"Expanded all rows" = Table.ExpandTableColumn(#"Removed Duplicates", "all rows", {"Call Date", "Bill To", "Shipper", "Ready", "Consignee", "Need", "Cont", "Pounds", "Miles", "Response", "Who", "Award", "Unit", "Mode", "Posted", "Responded", "Awarded", "Year", "Month", "Week #", "OD Lane Pairs", "Origin", "Destination", "Response (Y/N)", "Declined", "Award Confirmed", "Response Time", "Time Posted", "After Hours", "Posted During Operating Hours", "Bid During Operating Hours", "Weight (kgs)", "Cost Per KG", "Missed Kgs", "Estimated Revenue"}, {"Call Date", "Bill To", "Shipper", "Ready", "Consignee", "Need", "Cont", "Pounds", "Miles", "Response", "Who", "Award", "Unit", "Mode", "Posted", "Responded", "Awarded", "Year", "Month", "Week #", "OD Lane Pairs", "Origin", "Destination", "Response (Y/N)", "Declined", "Award Confirmed", "Response Time", "Time Posted", "After Hours", "Posted During Operating Hours", "Bid During Operating Hours", "Weight (kgs)", "Cost Per KG", "Missed Kgs", "Estimated Revenue"})
in
    #"Expanded all rows"