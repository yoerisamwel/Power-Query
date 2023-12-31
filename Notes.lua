Notes
if validation of GSDB Lane plan
= Table.AddColumn(#"Removed Columns", "Merge_column_key", each if [App B  Dest GSDB] = null then [Destination Code] & " " & [Lane ID] else [App B  Dest GSDB] & " " & [Lane ID])


else if [Destination GSDB Code] = true then 
if [Origin GSDB Code] = [Lane_check_table.Origin_GSDB_check] then 
if [Carrier_CW1_names.Ocean Carrier] = [Lane_check_table.Carrier_check] then "Lane ID info correct" 
else "Destination GSDB error" 
else "Origin GSDB error" 
else "Carrier info incorrect")

= Table.AddColumn(#"Added Custom", "Lane and GSDB check", each 
if List.NonNullCount({[#"RefNums (SRF)"],[Lane_check_table.Origin_GSDB_check],[Destination GSDB Code],[Lane_check_table.Destination_GSDB_check], [Carrier_CW1_names.Ocean Carrier], [Lane_check_table.Carrier_check]}) < 6 then "Contains Blank"
    else 
    if [Add_false_if_no_match] = true then 
    if [#"RefNums (SRF)"] is [Lane_check_table.Origin_GSDB_check] then 
    if [Carrier_CW1_names.Ocean Carrier] is [Lane_check_table.Carrier_check] then "Lane ID info correct" 
    else "Destination GSDB error" else "Origin GSDB error" else "Carrier info incorrect")


= Table.AddColumn(#"Expanded Lane_check_table", "Lane and GSDB check", each if Text.Contains([#"RefNums (SRF)"], [Lane_check_table.Origin_GSDB_check]) then if Text.Contains([Destination GSDB Code], [Lane_check_table.Destination_GSDB_check]) then if Text.Contains([Carrier_CW1_names.Ocean Carrier], [Lane_check_table.Carrier_check]) then "Lane ID info correct" else "Origin GSDB error" else "Destination GSDB error" else "Carrier name error")

AF_US
let
    //pulling in and cleaning the data from the shared excel file
    Source = Excel.Workbook(Web.Contents("https://dsvcorp.sharepoint.com/teams/FORD-GlobalAirfreightAward/Shared%20Documents/General/AOD%20AIR%20Bidding%20-%20Ford%20Production/Ford%20AIR%20Rate%20Card%20-%20Production.xlsx"), null, true),
    Custom1 = Source{[Item="AF US",Kind="Sheet"]}[Data],
    #"Removed Blank Rows" = Table.SelectRows(Custom1, each not List.IsEmpty(List.RemoveMatchingItems(Record.FieldValues(_), {"", null}))),
    #"Promoted Headers" = Table.PromoteHeaders(#"Removed Blank Rows", [PromoteAllScalars=true]),
    #"Changed Type" = Table.TransformColumnTypes(#"Promoted Headers",{{"DSV Lane ID", type text}, {"SERVICE", type text}, {"ORIGIN REGION", type text}, {"ORIGIN COUNTRY", type text}, {"DESTINATION COUNTRY", type text}, {"OD Pair", type text}, {"TRANSIT TIME REQUIREMENT (HRS)", type text}, {"CURRENCY", type text}, {"AIRFREIGHT MIN", Int64.Type}, {"<500KG", type number}, {"+500KG", type number}, {"+1000KG", type number}, {"FSC (PER KG)", type number}, {"SSC (PER KG)", type number}, {"Oversize Surcharge", type any}, {"Update Date", type any}, {"Update User", type text}, {"Comments ", type text}})
in
    #"Changed Type"

AF_MX
let
    Source = Excel.Workbook(Web.Contents("https://dsvcorp.sharepoint.com/teams/FORD-GlobalAirfreightAward/Shared%20Documents/General/AOD%20AIR%20Bidding%20-%20Ford%20Production/Ford%20AIR%20Rate%20Card%20-%20Production.xlsx"), null, true),
    Custom1 = Source{[Item="AF MX",Kind="Sheet"]}[Data],
    #"Removed Blank Rows" = Table.SelectRows(Custom1, each not List.IsEmpty(List.RemoveMatchingItems(Record.FieldValues(_), {"", null}))),
    #"Promoted Headers" = Table.PromoteHeaders(#"Removed Blank Rows", [PromoteAllScalars=true]),
    #"Changed Type" = Table.TransformColumnTypes(#"Promoted Headers",{{"3.5", type text}, {"SERVICE", type text}, {"ORIGIN REGION", type text}, {"ORIGIN COUNTRY", type text}, {"DESTINATION COUNTRY", type text}, {"OD Pair", type text}, {"TRANSIT TIME REQUIREMENT (HRS)", type text}, {"CURRENCY", type text}, {"AIRFREIGHT MIN", Int64.Type}, {"<100KG", type any}, {"<500KG", type number}, {"+500KG", type number}, {"+1000KG", type number}, {"FSC (PER KG)", type number}, {"SSC (PER KG)", Int64.Type}, {"Oversize Surcharge", type any}, {"Update Date", type date}, {"Update User", type text}, {"Comments ", type text}}),
    #"Renamed Columns" = Table.RenameColumns(#"Changed Type",{{"3.5", "DSV Lane ID"}})
in
    #"Renamed Columns"

data_pull
let
    //pulled in and cleaned table
    Source = Table.Combine({AF_MX, AF_US}),
    #"Added datetime stamp" = Table.AddColumn(Source, "Adding datetime stamp", each DateTime.LocalNow()),
    #"Extracted day" = Table.AddColumn(#"Added datetime stamp", "Extract day from datetimestamp", each Date.DayOfYear([Adding datetime stamp])),
    #"Extracted year" = Table.AddColumn(#"Extracted day", "Extract year of datetime stamp", each Date.Year([Adding datetime stamp])),
    #"Changed Type" = Table.TransformColumnTypes(#"Extracted year",{{"Extract day from datetimestamp", type text}, {"Extract year of datetime stamp", type text}}),
    #"Added Custom" = Table.AddColumn(#"Changed Type", "Primary_key", each [DSV Lane ID] & [SERVICE] & [Extract day from datetimestamp] & [Extract year of datetime stamp])
in
    #"Added Custom"

Source_table
let
    Source = Excel.CurrentWorkbook(){[Name="Table4"]}[Content],
    #"Changed Type" = Table.TransformColumnTypes(Source,{{"DSV Lane ID", type text}, {"SERVICE", type text}, {"ORIGIN REGION", type text}, {"ORIGIN COUNTRY", type text}, {"DESTINATION COUNTRY", type text}, {"OD Pair", type text}, {"TRANSIT TIME REQUIREMENT (HRS)", type text}, {"CURRENCY", type text}, {"AIRFREIGHT MIN", Int64.Type}, {"<100KG", type any}, {"<500KG", type number}, {"+500KG", type number}, {"+1000KG", type number}, {"FSC (PER KG)", type number}, {"SSC (PER KG)", type number}, {"Oversize Surcharge", type any}, {"Update Date", type text}, {"Update User", type text}, {"Comments ", type text}})
in
    #"Changed Type"

Append1
let
    Source = Table.Combine({Source_table, data_pull})
in
    Source