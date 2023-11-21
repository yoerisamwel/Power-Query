DSV shipment report sheet
let
    //Data source
    Source = Excel.Workbook(Web.Contents("https://dsvcorp.sharepoint.com/teams/FORDAnalytics/Shared%20Documents/General/FCSD%20export%20error%20check/EWR%20DSV%20Shipment%20Report%20(Transport%20Data)%20Friday,%2010%20November%202023%2013_47_21.XLSX"), null, true),
    FCSD_error_data_pull = Source{[Item="DSV Shipment Report",Kind="Sheet"]}[Data],
    #"Filtered Rows" = Table.SelectRows(FCSD_error_data_pull, each [Column3] <> null and [Column3] <> ""),
    #"Removed Columns" = Table.RemoveColumns(#"Filtered Rows",{"Column1", "Column2"}),
    #"Promoted Headers" = Table.PromoteHeaders(#"Removed Columns", [PromoteAllScalars=true]),
    #"Changed Type" = Table.TransformColumnTypes(#"Promoted Headers",{{"Shipment ID", type text}, {"House Ref", type text}, {"Trans", type text}, {"Mode", type text}, {"Origin", type text}, {"Destination", type text}, {"Shipment Dest ETA", type any}, {"Job Branch", type text}, {"Job Dept", type text}, {"INCO", type text}, {"Weight", type number}, {"UQ", type text}, {"Volume", type number}, {"UQ_1", type text}, {"UQ_2", type text}, {"Chargeable", type number}, {"Actual Pickup (Date Only)", type any}, {"Consignor Name", type text}, {"Consignor Country", type text}, {"Destination GSDB Code", type text}, {"Consignee Name", type text}, {"Consignee Country", type text}, {"Carrier Name", type text}, {"Load(ARV - LEG)", type text}, {"Disch(ARV - LEG)", type text}, {"Vessel (ARV - LEG)", type text}, {"Job Status", type text}, {"Containers", type text}, {"Job Opened", type datetime}, {"FMC Reference", type text}, {"Shipment Origin ETD", type any}, {"ETD Load(ARV - LEG)", type any}, {"First SEA Leg ATD", type datetime}, {"RefNums (SCN)", type text}, {"RefNums (INO)", type text}, {"RefNums (LAI)", type text}, {"RefNums (SRF)", type text}}),
    #"Trimmed Text Lane_ID and Carrier and Destination" = Table.TransformColumns(#"Changed Type",{{"RefNums (LAI)", Text.Trim, type text}, {"Carrier Name", Text.Trim, type text}, {"Destination GSDB Code", Text.Trim, type text}}),
    #"Merged carrier names" = Table.NestedJoin(#"Trimmed Text Lane_ID and Carrier and Destination", {"Carrier Name"}, Carrier_CW1_names, {"Carrier name"}, "Carrier_CW1_names", JoinKind.LeftOuter),
    #"Expanded Carrier_CW1_names" = Table.ExpandTableColumn(#"Merged carrier names", "Carrier_CW1_names", {"Carrier name", "Ocean Carrier", "Carrier Code"}, {"Carrier_CW1_names.Carrier name", "Carrier_CW1_names.Ocean Carrier", "Carrier_CW1_names.Carrier Code"}),
    #"Blank check RefNums (SCN)" = Table.AddColumn(#"Expanded Carrier_CW1_names", "Blank check RefNums (SCN)", each if [#"RefNums (SCN)"] is null then true else false),
    #"Blank check RefNums (INO)" = Table.AddColumn(#"Blank check RefNums (SCN)", "Blank check RefNums (INO)", each if [#"RefNums (INO)"] is null then true else false),
    #"Blank check RefNums (LAI)" = Table.AddColumn(#"Blank check RefNums (INO)", "Blank check RefNums (LAI)", each if [#"RefNums (LAI)"] is null then true else false),
    #"Blank check RefNums (SRF)" = Table.AddColumn(#"Blank check RefNums (LAI)", "Blank check RefNums (SRF)", each if [#"RefNums (SRF)"] is null then true else false),
    #"Blank check Destination GSDB Code" = Table.AddColumn(#"Blank check RefNums (SRF)", "Blank check Destination GSDB Code", each if [#"Destination GSDB Code"] is null then true else false),
    #"Blacnk check First SEA Leg ATD" = Table.AddColumn(#"Blank check Destination GSDB Code", "Blank check First SEA Leg ATD", each if [#"First SEA Leg ATD"] is null then true else false),
    #"Blank check ETD Load(ARV - LEG)" = Table.AddColumn(#"Blacnk check First SEA Leg ATD", "Blank check ETD Load(ARV - LEG)", each if [#"ETD Load(ARV - LEG)"] is null then true else false),
    #"Blank check Shipment Origin ETD" = Table.AddColumn(#"Blank check ETD Load(ARV - LEG)", "Blank check Shipment Origin ETD", each if [#"Shipment Origin ETD"] is null then true else false),
    #"Blank check Job Opened" = Table.AddColumn(#"Blank check Shipment Origin ETD", "Blank check Job Opened", each if [#"Job Opened"] is null then true else false),
    #"Blank check Containers" = Table.AddColumn(#"Blank check Job Opened", "Blank check Containers", each if [#"Containers"] is null then true else false),
    #"Blank check FMC Reference" = Table.AddColumn(#"Blank check Containers", "Blank check FMC Reference", each if [#"FMC Reference"] is null then true else false),
    #"Blank check Job Status" = Table.AddColumn(#"Blank check FMC Reference", "Blank check Job Status", each if [#"Job Status"] is null then true else false),
    #"Blank check Vessel (ARV - LEG)" = Table.AddColumn(#"Blank check Job Status", "Blank check Vessel (ARV - LEG)", each if [#"Vessel (ARV - LEG)"] is null then true else false),
    #"Blank check Disch(ARV - LEG)" = Table.AddColumn(#"Blank check Vessel (ARV - LEG)", "Blank check Disch(ARV - LEG)", each if [#"Disch(ARV - LEG)"] is null then true else false),
    #"Blank check Load(ARV - LEG)" = Table.AddColumn(#"Blank check Disch(ARV - LEG)", "Blank check Load(ARV - LEG)", each if [#"Load(ARV - LEG)"] is null then true else false),
    #"Blank check Carrier Name" = Table.AddColumn(#"Blank check Load(ARV - LEG)", "Blank check Carrier Name", each if [#"Carrier Name"] is null then true else false),
    #"Blank check Consignee Country" = Table.AddColumn(#"Blank check Carrier Name", "Blank check Consignee Country", each if [#"Consignee Country"] is null then true else false),
    #"Blank check Consignee Name" = Table.AddColumn(#"Blank check Consignee Country", "Blank check Consignee Name", each if [#"Consignee Name"] is null then true else false),
    #"Blank check Consignor Country" = Table.AddColumn(#"Blank check Consignee Name", "Blank check Consignor Country", each if [#"Consignor Country"] is null then true else false),
    #"Blank check Consignor Name" = Table.AddColumn(#"Blank check Consignor Country", "Blank check Consignor Name", each if [#"Consignor Name"] is null then true else false),
    #"Blank check Actual Pickup (Date Only)" = Table.AddColumn(#"Blank check Consignor Name", "Blank check Actual Pickup (Date Only)", each if [#"Actual Pickup (Date Only)"] is null then true else false),
    #"Blank check Chargeable" = Table.AddColumn(#"Blank check Actual Pickup (Date Only)", "Blank check Chargeable", each if [#"Chargeable"] is null then true else false),
    #"Blank check UQ_2" = Table.AddColumn(#"Blank check Chargeable", "Blank check UQ_2", each if [#"UQ_2"] is null then true else false),
    #"Blank check UQ_1" = Table.AddColumn(#"Blank check UQ_2", "Blank check UQ_1", each if [#"UQ_1"] is null then true else false),
    #"Blank check Volume" = Table.AddColumn(#"Blank check UQ_1", "Blank check Volume", each if [#"Volume"] is null then true else false),
    #"Blank check UQ" = Table.AddColumn(#"Blank check Volume", "Blank check UQ", each if [#"UQ"] is null then true else false),
    #"Blank check Weight" = Table.AddColumn(#"Blank check UQ", "Blank check Weight", each if [#"Weight"] is null then true else false),
    #"Blank check INCO" = Table.AddColumn(#"Blank check Weight", "Blank check INCO", each if [#"INCO"] is null then true else false),
    #"Blank check Job Dept" = Table.AddColumn(#"Blank check INCO", "Blank check Job Dept", each if [#"Job Dept"] is null then true else false),
    #"Blank check Job Branch" = Table.AddColumn(#"Blank check Job Dept", "Blank check Job Branch", each if [#"Job Branch"] is null then true else false),
    #"Blank check Shipment Dest ETA" = Table.AddColumn(#"Blank check Job Branch", "Blank check Shipment Dest ETA", each if [#"Shipment Dest ETA"] is null then true else false),
    #"Blank check Destination" = Table.AddColumn(#"Blank check Shipment Dest ETA", "Blank check Destination", each if [#"Destination"] is null then true else false),
    #"Blank check Origin" = Table.AddColumn(#"Blank check Destination", "Blank check Origin", each if [#"Origin"] is null then true else false),
    #"Blank check Mode" = Table.AddColumn(#"Blank check Origin", "Blank check Mode", each if [#"Mode"] is null then true else false),
    #"Blank check Trans" = Table.AddColumn(#"Blank check Mode", "Blank check Trans", each if [#"Trans"] is null then true else false),
    #"Blank check House Ref" = Table.AddColumn(#"Blank check Trans", "Blank check House Ref", each if [#"House Ref"] is null then true else false),
    #"Blank check Shipment ID" = Table.AddColumn(#"Blank check House Ref", "Blank check Shipment ID", each if [#"Shipment ID"] is null then true else false),
    #"Renamed LANE_ID" = Table.RenameColumns(#"Blank check Shipment ID",{{"RefNums (LAI)", "LANE_ID"}}),
    #"Merged Queries" = Table.NestedJoin(#"Renamed LANE_ID", {"LANE_ID"}, Lane_check_table, {"LANE ID"}, "Lane_check_table", JoinKind.LeftOuter),
    #"Expanded Lane_check_table" = Table.ExpandTableColumn(#"Merged Queries", "Lane_check_table", {"LANE ID", "Origin_GSDB_check", "Destination_GSDB_check", "Carrier_check"}, {"Lane_check_table.LANE ID", "Lane_check_table.Origin_GSDB_check", "Lane_check_table.Destination_GSDB_check", "Lane_check_table.Carrier_check"}),
    #"Added merge_key" = Table.AddColumn(#"Expanded Lane_check_table", "Merge_column_key", each [Destination GSDB Code] & " " & [LANE_ID]),
    #"Changed merge_key" = Table.TransformColumnTypes(#"Added merge_key",{{"Merge_column_key", type text}}),
    #"Merged Queries1" = Table.FuzzyNestedJoin(#"Changed merge_key", {"Merge_column_key"}, Combinations_CW1_codes, {"Merge_column_key"}, "Combinations_CW1_codes", JoinKind.LeftOuter, [IgnoreCase=true, IgnoreSpace=true]),
    #"Expanded Combinations_CW1_codes" = Table.ExpandTableColumn(#"Merged Queries1", "Combinations_CW1_codes", {"Merge_column_key", "Dest_GSDB_valid"}, {"Combinations_CW1_codes.Merge_column_key", "Combinations_CW1_codes.Dest_GSDB_valid"}),
    #"Added add false if no match" = Table.AddColumn(#"Expanded Combinations_CW1_codes", "Add_false_if_no_match", each if [Combinations_CW1_codes.Dest_GSDB_valid] = true then true else false),
    #"Uppercased Text" = Table.TransformColumns(#"Added add false if no match",{{"Lane_check_table.Origin_GSDB_check", Text.Upper, type text}}),
    #"Trimmed Text" = Table.TransformColumns(#"Uppercased Text",{{"Lane_check_table.Origin_GSDB_check", Text.Trim, type text}}),
    #"Cleaned Text" = Table.TransformColumns(#"Trimmed Text",{{"Lane_check_table.Origin_GSDB_check", Text.Clean, type text}}),
    #"Uppercased Text1" = Table.TransformColumns(#"Cleaned Text",{{"RefNums (SRF)", Text.Upper, type text}}),
    #"Trimmed Text1" = Table.TransformColumns(#"Uppercased Text1",{{"RefNums (SRF)", Text.Trim, type text}}),
    #"Cleaned Text1" = Table.TransformColumns(#"Trimmed Text1",{{"RefNums (SRF)", Text.Clean, type text}}),
    Lane_GSDB_error_check = Table.AddColumn(#"Cleaned Text1", "Lane and GSDB check", each if List.NonNullCount({[#"RefNums (SRF)"],[Lane_check_table.Origin_GSDB_check],[Destination GSDB Code],[Lane_check_table.Destination_GSDB_check], [Carrier_CW1_names.Ocean Carrier], [Lane_check_table.Carrier_check]}) < 6 then "Contains Blank"
    else if [Add_false_if_no_match] = true then if [#"RefNums (SRF)"] = [Lane_check_table.Origin_GSDB_check] then if [Carrier_CW1_names.Ocean Carrier] = [Lane_check_table.Carrier_check] then "Lane ID info correct" else "Carrier info incorrect" else "Origin GSDB error" else "Destination GSDB error")
in
    Lane_GSDB_error_check

Trip budget
let
    //Data source
    Source = Excel.Workbook(Web.Contents("https://dsvcorp.sharepoint.com/teams/FORDAnalytics/Shared%20Documents/General/FCSD%20export%20error%20check/IDO_trip_budget.xlsx"), null, true),
    Trip_budget = Source{[Item="Trip_budget",Kind="Sheet"]}[Data],
    #"Promoted Headers" = Table.PromoteHeaders(Trip_budget, [PromoteAllScalars=true]),
    #"Changed Type" = Table.TransformColumnTypes(#"Promoted Headers",{{"LANE ID", type text}, {"Origin GSDB", type text}, {"Origin Name", type text}, {"Destination GSDB", type any}, {"Carrier", type text}, {"Origin Port", type text}, {"Transload Port", type text}, {"Destination Port", type text}, {"Total TT", Int64.Type}, {"CL", Int64.Type}, {"PU", Int64.Type}, {"VS", Int64.Type}, {"VA", Int64.Type}, {"Grey", Int64.Type}, {"sum_column", Int64.Type}, {"check_column", Int64.Type}})
in
    #"Changed Type"

Lane check Table

let
    Source = Table.FromRows(Json.Document(Binary.Decompress(Binary.FromText("i44FAA==", BinaryEncoding.Base64), Compression.Deflate)), let _t = ((type nullable text) meta [Serialized.Text = true]) in type table [#"LANE ID" = _t, #"Origin GSDB" = _t, #"Origin Name" = _t, #"Destination GSDB" = _t, Carrier = _t, #"Origin Port" = _t, #"Transload Port" = _t, #"Destination Port" = _t, #"Total TT" = _t, CL = _t, PU = _t, VS = _t, VA = _t, Grey = _t, sum_column = _t, check_column = _t]),
    #"Appended Query" = Table.Combine({Source, Trip_budget}),
    #"Changed Type" = Table.TransformColumnTypes(#"Appended Query",{{"LANE ID", type text}, {"Origin GSDB", type text}, {"Origin Name", type text}, {"Destination GSDB", type text}, {"Carrier", type text}, {"Origin Port", type text}, {"Transload Port", type text}, {"Destination Port", type text}, {"Total TT", type text}, {"CL", type text}, {"PU", type text}, {"VS", type text}, {"VA", type text}, {"Grey", type text}, {"sum_column", type text}, {"check_column", type text}}),
    #"Removed unneeded Columns" = Table.RemoveColumns(#"Changed Type",{"Origin Name", "Origin Port", "Transload Port", "Destination Port", "Total TT", "CL", "PU", "VS", "VA", "Grey", "sum_column", "check_column"}),
    #"Renamed to check" = Table.RenameColumns(#"Removed unneeded Columns",{{"Origin GSDB", "Origin_GSDB_check"}, {"Destination GSDB", "Destination_GSDB_check"}, {"Carrier", "Carrier_check"}})
in
    #"Renamed to check"

Carrier CW1 names
let
    Source = Table.FromRows(Json.Document(Binary.Decompress(Binary.FromText("TU9BboMwEPzKiBOVkqqiL3DMCqxgm9qmKUU5IIFSpJagKJf8vgskUi47uzOe8W7TRFILyEwjljqtXqLNg1i6tIqOmybKhd5VLoOvUmwhrS3JiaCswUGFHFI4p8hBeZS58JTCVgG7GmRSfLxvk7ck4binFJ64ruHWEIdaScLAUDhYtwd9lY68R8xiPS/FuNZ68Wgv2aP7brj2l0s79u0I/zNM0zCeIM9/UzveEPOr5SLGta4fakHO72fmqbkf2k7taVv8nm8d4rxY7bkoRTZjcQ8INhC0cCooPa81zwzmMyzyt9Iort0rU9w+hOM/", BinaryEncoding.Base64), Compression.Deflate)), let _t = ((type nullable text) meta [Serialized.Text = true]) in type table [#"Carrier name" = _t, #"Ocean Carrier" = _t, #"Carrier Code" = _t]),
    #"Changed Type" = Table.TransformColumnTypes(Source,{{"Carrier name", type text}, {"Ocean Carrier", type text}, {"Carrier Code", type text}})
in
    #"Changed Type"

Error output sheet
let
    Source = #"DSV Shipment Report",
    // Group by "Blank check RefNums (SCN)" where "BooleanColumn" is true
    GroupedTable = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable = Table.SelectRows(GroupedTable, each [GroupedData][#"Blank check RefNums (SCN)"]{0} = true),
    #"Expanded GroupedData" = Table.ExpandTableColumn(FilteredTable, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type" = Table.AddColumn(#"Expanded GroupedData", "Error", each "Blank check RefNums (SCN)"),
    #"Removed Columns" = Table.RemoveColumns(#"Added error type",{"GroupedData.BooleanColumn"}),

    // Group by "Blank check RefNums (INO)" where "BooleanColumn" is true
    GroupedTable2 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable2 = Table.SelectRows(GroupedTable2, each [GroupedData][#"Blank check RefNums (INO)"]{0} = true),
    #"Expanded GroupedData2" = Table.ExpandTableColumn(FilteredTable2, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 2" = Table.AddColumn(#"Expanded GroupedData2", "Error", each "Blank check RefNums (INO)"),
    #"Removed Columns2" = Table.RemoveColumns(#"Added error type 2",{"GroupedData.BooleanColumn"}),
    //Appending datasets 1 "Blank check RefNums (SCN)" and "Blank check RefNums (INO)"
    #"Appended" = Table.Combine({#"Removed Columns", #"Removed Columns2"}),

    // Group by "Blank check RefNums (LAI)" where "BooleanColumn" is true
    GroupedTable3 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable3 = Table.SelectRows(GroupedTable3, each [GroupedData][#"Blank check RefNums (LAI)"]{0} = true),
    #"Expanded GroupedData3" = Table.ExpandTableColumn(FilteredTable2, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 3" = Table.AddColumn(#"Expanded GroupedData3", "Error", each "Blank check RefNums (LAI)"),
    #"Removed Columns3" = Table.RemoveColumns(#"Added error type 3",{"GroupedData.BooleanColumn"}),
    //Appending datasets 2 "Blank check RefNums (LAI)"
    #"Appended2" = Table.Combine({#"Appended", #"Removed Columns3"}),

        // Group by "Blank check RefNums (SRF)" where "BooleanColumn" is true
    GroupedTable4 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable4 = Table.SelectRows(GroupedTable4, each [GroupedData][#"Blank check RefNums (SRF)"]{0} = true),
    #"Expanded GroupedData4" = Table.ExpandTableColumn(FilteredTable4, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 4" = Table.AddColumn(#"Expanded GroupedData4", "Error", each "Blank check RefNums (SRF)"),
    #"Removed Columns4" = Table.RemoveColumns(#"Added error type 4",{"GroupedData.BooleanColumn"}),
    //Appending datasets 3 "Blank check RefNums (SRF)"
    #"Appended3" = Table.Combine({#"Appended2", #"Removed Columns4"}),

    // Group by "Blank check Destination GSDB Code" where "BooleanColumn" is true
    GroupedTable5 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable5 = Table.SelectRows(GroupedTable5, each [GroupedData][#"Blank check Destination GSDB Code"]{0} = true),
    #"Expanded GroupedData5" = Table.ExpandTableColumn(FilteredTable5, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 5" = Table.AddColumn(#"Expanded GroupedData5", "Error", each "Blank check Destination GSDB Code"),
    #"Removed Columns5" = Table.RemoveColumns(#"Added error type 5",{"GroupedData.BooleanColumn"}),
    //Appending datasets 4 "Blank check Destination GSDB Code"
    #"Appended4" = Table.Combine({#"Appended3", #"Removed Columns5"}),

    // Group by "Blank check RefNums (INO)" where "BooleanColumn" is true
    GroupedTable6 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable6 = Table.SelectRows(GroupedTable6, each [GroupedData][#"Blank check First SEA Leg ATD"]{0} = true),
    #"Expanded GroupedData6" = Table.ExpandTableColumn(FilteredTable6, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 6" = Table.AddColumn(#"Expanded GroupedData6", "Error", each "Blank check First SEA Leg ATD"),
    #"Removed Columns6" = Table.RemoveColumns(#"Added error type 6",{"GroupedData.BooleanColumn"}),
    //Appending datasets 5 Blank check First SEA Leg ATD
    #"Appended5" = Table.Combine({#"Appended3", #"Removed Columns6"}),

    // Group by "Blank check ETD Load(ARV - LEG)" where "BooleanColumn" is true
    GroupedTable7 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable7 = Table.SelectRows(GroupedTable7, each [GroupedData][#"Blank check ETD Load(ARV - LEG)"]{0} = true),
    #"Expanded GroupedData7" = Table.ExpandTableColumn(FilteredTable7, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 7" = Table.AddColumn(#"Expanded GroupedData7", "Error", each "Blank check ETD Load(ARV - LEG)"),
    #"Removed Columns7" = Table.RemoveColumns(#"Added error type 7",{"GroupedData.BooleanColumn"}),
    //Appending datasets 6 Blank check ETD Load(ARV - LEG)
    #"Appended6" = Table.Combine({#"Appended5", #"Removed Columns7"}),

    // Group by "Blank check Shipment Origin ETD" where "BooleanColumn" is true
    GroupedTable8 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable8 = Table.SelectRows(GroupedTable8, each [GroupedData][#"Blank check Shipment Origin ETD"]{0} = true),
    #"Expanded GroupedData8" = Table.ExpandTableColumn(FilteredTable8, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 8" = Table.AddColumn(#"Expanded GroupedData8", "Error", each "Blank check Shipment Origin ETD"),
    #"Removed Columns8" = Table.RemoveColumns(#"Added error type 8",{"GroupedData.BooleanColumn"}),
    //Appending datasets 7 Blank check Shipment Origin ETD
    #"Appended7" = Table.Combine({#"Appended6", #"Removed Columns8"}),

    // Group by "Blank check Job Opened" where "BooleanColumn" is true
    GroupedTable9 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable9 = Table.SelectRows(GroupedTable9, each [GroupedData][#"Blank check Job Opened"]{0} = true),
    #"Expanded GroupedData9" = Table.ExpandTableColumn(FilteredTable9, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 9" = Table.AddColumn(#"Expanded GroupedData9", "Error", each "Blank check Job Opened"),
    #"Removed Columns9" = Table.RemoveColumns(#"Added error type 9",{"GroupedData.BooleanColumn"}),
    //Appending datasets 8 Blank check Job Opened
    #"Appended8" = Table.Combine({#"Appended7", #"Removed Columns9"}),


    // Group by "Blank check Containers" where "BooleanColumn" is true
    GroupedTable10 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable10 = Table.SelectRows(GroupedTable10, each [GroupedData][#"Blank check Containers"]{0} = true),
    #"Expanded GroupedData10" = Table.ExpandTableColumn(FilteredTable10, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 10" = Table.AddColumn(#"Expanded GroupedData10", "Error", each "Blank check Containers"),
    #"Removed Columns10" = Table.RemoveColumns(#"Added error type 10",{"GroupedData.BooleanColumn"}),
    //Appending datasets 9 Blank check Containers
    #"Appended9" = Table.Combine({#"Appended8", #"Removed Columns10"}),

    // Group by "Blank check FMC Reference" where "BooleanColumn" is true
    GroupedTable11 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable11 = Table.SelectRows(GroupedTable11, each [GroupedData][#"Blank check FMC Reference"]{0} = true),
    #"Expanded GroupedData11" = Table.ExpandTableColumn(FilteredTable11, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 11" = Table.AddColumn(#"Expanded GroupedData11", "Error", each "Blank check FMC Reference"),
    #"Removed Columns11" = Table.RemoveColumns(#"Added error type 11",{"GroupedData.BooleanColumn"}),
    //Appending datasets 10 Blank check FMC Reference
    #"Appended10" = Table.Combine({#"Appended9", #"Removed Columns11"}),

    // Group by "Blank check Job Status" where "BooleanColumn" is true
    GroupedTable12 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable12 = Table.SelectRows(GroupedTable12, each [GroupedData][#"Blank check Job Status"]{0} = true),
    #"Expanded GroupedData12" = Table.ExpandTableColumn(FilteredTable12, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 12" = Table.AddColumn(#"Expanded GroupedData12", "Error", each "Blank check Job Status"),
    #"Removed Columns12" = Table.RemoveColumns(#"Added error type 12",{"GroupedData.BooleanColumn"}),
    //Appending datasets 11 Blank check Job Status
    #"Appended11" = Table.Combine({#"Appended10", #"Removed Columns12"}),

    // Group by "Blank check Vessel (ARV - LEG)" where "BooleanColumn" is true
    GroupedTable13 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable13 = Table.SelectRows(GroupedTable13, each [GroupedData][#"Blank check Vessel (ARV - LEG)"]{0} = true),
    #"Expanded GroupedData13" = Table.ExpandTableColumn(FilteredTable13, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 13" = Table.AddColumn(#"Expanded GroupedData13", "Error", each "Blank check Vessel (ARV - LEG)"),
    #"Removed Columns13" = Table.RemoveColumns(#"Added error type 13",{"GroupedData.BooleanColumn"}),
    //Appending datasets 12 Blank check Vessel (ARV - LEG)
    #"Appended12" = Table.Combine({#"Appended11", #"Removed Columns13"}),

    // Group by "Blank check UQ_1" where "BooleanColumn" is true
    GroupedTable14 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable14 = Table.SelectRows(GroupedTable14, each [GroupedData][#"Blank check UQ_1"]{0} = true),
    #"Expanded GroupedData14" = Table.ExpandTableColumn(FilteredTable14, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 14" = Table.AddColumn(#"Expanded GroupedData14", "Error", each "Blank check UQ_1"),
    #"Removed Columns14" = Table.RemoveColumns(#"Added error type 14",{"GroupedData.BooleanColumn"}),
    //Appending datasets 13 Blank check UQ_1
    #"Appended13" = Table.Combine({#"Appended12", #"Removed Columns14"}),

    // Group by "Blank check Load(ARV - LEG)" where "BooleanColumn" is true
    GroupedTable15 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable15 = Table.SelectRows(GroupedTable15, each [GroupedData][#"Blank check Load(ARV - LEG)"]{0} = true),
    #"Expanded GroupedData15" = Table.ExpandTableColumn(FilteredTable15, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 15" = Table.AddColumn(#"Expanded GroupedData15", "Error", each "Blank check Load(ARV - LEG)"),
    #"Removed Columns15" = Table.RemoveColumns(#"Added error type 15",{"GroupedData.BooleanColumn"}),
    //Appending datasets 14 Blank check Load(ARV - LEG)
    #"Appended14" = Table.Combine({#"Appended13", #"Removed Columns15"}),

    // Group by "Blank check Carrier Name" where "BooleanColumn" is true
    GroupedTable16 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable16 = Table.SelectRows(GroupedTable16, each [GroupedData][#"Blank check Carrier Name"]{0} = true),
    #"Expanded GroupedData16" = Table.ExpandTableColumn(FilteredTable16, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 16" = Table.AddColumn(#"Expanded GroupedData16", "Error", each "Blank check Carrier Name"),
    #"Removed Columns16" = Table.RemoveColumns(#"Added error type 16",{"GroupedData.BooleanColumn"}),
    //Appending datasets 15 Blank check Carrier Name
    #"Appended15" = Table.Combine({#"Appended14", #"Removed Columns16"}),

    // Group by "Blank check Consignee Country" where "BooleanColumn" is true
    GroupedTable17 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable17 = Table.SelectRows(GroupedTable17, each [GroupedData][#"Blank check Consignee Country"]{0} = true),
    #"Expanded GroupedData17" = Table.ExpandTableColumn(FilteredTable17, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 17" = Table.AddColumn(#"Expanded GroupedData17", "Error", each "Blank check Consignee Country"),
    #"Removed Columns17" = Table.RemoveColumns(#"Added error type 17",{"GroupedData.BooleanColumn"}),
    //Appending datasets 16 Blank check Consignee Country
    #"Appended16" = Table.Combine({#"Appended15", #"Removed Columns17"}),

    // Group by "Blank check Consignee Name" where "BooleanColumn" is true
    GroupedTable18 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable18 = Table.SelectRows(GroupedTable18, each [GroupedData][#"Blank check Consignee Name"]{0} = true),
    #"Expanded GroupedData18" = Table.ExpandTableColumn(FilteredTable18, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 18" = Table.AddColumn(#"Expanded GroupedData18", "Error", each "Blank check Consignee Name"),
    #"Removed Columns18" = Table.RemoveColumns(#"Added error type 18",{"GroupedData.BooleanColumn"}),
    //Appending datasets 17 Blank check Consignee Name
    #"Appended17" = Table.Combine({#"Appended16", #"Removed Columns18"}),

    // Group by "Blank check Consignor Country" where "BooleanColumn" is true
    GroupedTable19 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable19 = Table.SelectRows(GroupedTable19, each [GroupedData][#"Blank check Consignor Country"]{0} = true),
    #"Expanded GroupedData19" = Table.ExpandTableColumn(FilteredTable19, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 19" = Table.AddColumn(#"Expanded GroupedData19", "Error", each "Blank check Consignor Country"),
    #"Removed Columns19" = Table.RemoveColumns(#"Added error type 19",{"GroupedData.BooleanColumn"}),
    //Appending datasets 18 Blank check Consignor Country
    #"Appended18" = Table.Combine({#"Appended17", #"Removed Columns19"}),

    // Group by "Blank check Consignor Country" where "BooleanColumn" is true
    GroupedTable20 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable20 = Table.SelectRows(GroupedTable20, each [GroupedData][#"Blank check Consignor Country"]{0} = true),
    #"Expanded GroupedData20" = Table.ExpandTableColumn(FilteredTable20, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 20" = Table.AddColumn(#"Expanded GroupedData20", "Error", each "Blank check Consignor Country"),
    #"Removed Columns20" = Table.RemoveColumns(#"Added error type 20",{"GroupedData.BooleanColumn"}),
    //Appending datasets 19 Blank check Consignor Country
    #"Appended19" = Table.Combine({#"Appended18", #"Removed Columns20"}),

    // Group by "Blank check Consignor Name" where "BooleanColumn" is true
    GroupedTable21 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable21 = Table.SelectRows(GroupedTable21, each [GroupedData][#"Blank check Consignor Name"]{0} = true),
    #"Expanded GroupedData21" = Table.ExpandTableColumn(FilteredTable21, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 21" = Table.AddColumn(#"Expanded GroupedData21", "Error", each "Blank check Consignor Name"),
    #"Removed Columns21" = Table.RemoveColumns(#"Added error type 21",{"GroupedData.BooleanColumn"}),
    //Appending datasets 20 Blank check Consignor Name
    #"Appended20" = Table.Combine({#"Appended19", #"Removed Columns21"}),

    // Group by "Blank check Actual Pickup (Date Only)" where "BooleanColumn" is true
    GroupedTable22 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable22 = Table.SelectRows(GroupedTable22, each [GroupedData][#"Blank check Actual Pickup (Date Only)"]{0} = true),
    #"Expanded GroupedData22" = Table.ExpandTableColumn(FilteredTable22, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 22" = Table.AddColumn(#"Expanded GroupedData22", "Error", each "Blank check Actual Pickup (Date Only)"),
    #"Removed Columns22" = Table.RemoveColumns(#"Added error type 22",{"GroupedData.BooleanColumn"}),
    //Appending datasets 21 Blank check Actual Pickup (Date Only)
    #"Appended21" = Table.Combine({#"Appended20", #"Removed Columns22"}),

    // Group by "Blank check Chargeable" where "BooleanColumn" is true
    GroupedTable23 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable23 = Table.SelectRows(GroupedTable23, each [GroupedData][#"Blank check Chargeable"]{0} = true),
    #"Expanded GroupedData23" = Table.ExpandTableColumn(FilteredTable23, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 23" = Table.AddColumn(#"Expanded GroupedData23", "Error", each "Blank check Chargeable"),
    #"Removed Columns23" = Table.RemoveColumns(#"Added error type 23",{"GroupedData.BooleanColumn"}),
    //Appending datasets 22 Blank check Chargeable
    #"Appended22" = Table.Combine({#"Appended21", #"Removed Columns23"}),

    // Group by "Blank check UQ_2" where "BooleanColumn" is true
    GroupedTable24 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable24 = Table.SelectRows(GroupedTable24, each [GroupedData][#"Blank check UQ_2"]{0} = true),
    #"Expanded GroupedData24" = Table.ExpandTableColumn(FilteredTable24, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 24" = Table.AddColumn(#"Expanded GroupedData24", "Error", each "Blank check UQ_2"),
    #"Removed Columns24" = Table.RemoveColumns(#"Added error type 24",{"GroupedData.BooleanColumn"}),
    //Appending datasets 23 Blank check UQ_2
    #"Appended23" = Table.Combine({#"Appended22", #"Removed Columns24"}),

    // Group by "Blank check Volume" where "BooleanColumn" is true
    GroupedTable25 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable25 = Table.SelectRows(GroupedTable25, each [GroupedData][#"Blank check Volume"]{0} = true),
    #"Expanded GroupedData25" = Table.ExpandTableColumn(FilteredTable25, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 25" = Table.AddColumn(#"Expanded GroupedData25", "Error", each "Blank check Volume"),
    #"Removed Columns25" = Table.RemoveColumns(#"Added error type 25",{"GroupedData.BooleanColumn"}),
    //Appending datasets 24 Blank check Volume
    #"Appended24" = Table.Combine({#"Appended23", #"Removed Columns25"}),

    // Group by "Blank check UQ" where "BooleanColumn" is true
    GroupedTable26 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable26 = Table.SelectRows(GroupedTable26, each [GroupedData][#"Blank check UQ"]{0} = true),
    #"Expanded GroupedData26" = Table.ExpandTableColumn(FilteredTable26, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 26" = Table.AddColumn(#"Expanded GroupedData26", "Error", each "Blank check UQ"),
    #"Removed Columns26" = Table.RemoveColumns(#"Added error type 26",{"GroupedData.BooleanColumn"}),
    //Appending datasets 25 Blank check UQ_2
    #"Appended25" = Table.Combine({#"Appended24", #"Removed Columns26"}),

    // Group by "Blank check Weight" where "BooleanColumn" is true
    GroupedTable27 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable27 = Table.SelectRows(GroupedTable27, each [GroupedData][#"Blank check Weight"]{0} = true),
    #"Expanded GroupedData27" = Table.ExpandTableColumn(FilteredTable27, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 27" = Table.AddColumn(#"Expanded GroupedData27", "Error", each "Blank check Weight"),
    #"Removed Columns27" = Table.RemoveColumns(#"Added error type 27",{"GroupedData.BooleanColumn"}),
    //Appending datasets 26 Blank check Weight
    #"Appended26" = Table.Combine({#"Appended25", #"Removed Columns27"}),

    // Group by "Blank check INCO" where "BooleanColumn" is true
    GroupedTable28 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable28 = Table.SelectRows(GroupedTable28, each [GroupedData][#"Blank check INCO"]{0} = true),
    #"Expanded GroupedData28" = Table.ExpandTableColumn(FilteredTable28, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 28" = Table.AddColumn(#"Expanded GroupedData28", "Error", each "Blank check INCO"),
    #"Removed Columns28" = Table.RemoveColumns(#"Added error type 28",{"GroupedData.BooleanColumn"}),
    //Appending datasets 27 Blank check INCO
    #"Appended27" = Table.Combine({#"Appended26", #"Removed Columns28"}),

    // Group by "Blank check Job Dept" where "BooleanColumn" is true
    GroupedTable29 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable29 = Table.SelectRows(GroupedTable29, each [GroupedData][#"Blank check Job Dept"]{0} = true),
    #"Expanded GroupedData29" = Table.ExpandTableColumn(FilteredTable29, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 29" = Table.AddColumn(#"Expanded GroupedData29", "Error", each "Blank check Job Dept"),
    #"Removed Columns29" = Table.RemoveColumns(#"Added error type 29",{"GroupedData.BooleanColumn"}),
    //Appending datasets 28 Blank check Job Dept
    #"Appended28" = Table.Combine({#"Appended27", #"Removed Columns29"}),

    // Group by "Blank check Job Branch" where "BooleanColumn" is true
    GroupedTable30 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable30 = Table.SelectRows(GroupedTable30, each [GroupedData][#"Blank check Job Branch"]{0} = true),
    #"Expanded GroupedData30" = Table.ExpandTableColumn(FilteredTable30, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 30" = Table.AddColumn(#"Expanded GroupedData30", "Error", each "Blank check Job Branch"),
    #"Removed Columns30" = Table.RemoveColumns(#"Added error type 30",{"GroupedData.BooleanColumn"}),
    //Appending datasets 29 Blank check Job Branch
    #"Appended29" = Table.Combine({#"Appended28", #"Removed Columns30"}),

    // Group by "Blank check Shipment Dest ETA" where "BooleanColumn" is true
    GroupedTable31 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable31 = Table.SelectRows(GroupedTable31, each [GroupedData][#"Blank check Shipment Dest ETA"]{0} = true),
    #"Expanded GroupedData31" = Table.ExpandTableColumn(FilteredTable31, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 31" = Table.AddColumn(#"Expanded GroupedData31", "Error", each "Blank check Shipment Dest ETA"),
    #"Removed Columns31" = Table.RemoveColumns(#"Added error type 31",{"GroupedData.BooleanColumn"}),
    //Appending datasets 30 Blank check Shipment Dest ETA
    #"Appended30" = Table.Combine({#"Appended29", #"Removed Columns31"}),

    // Group by "Blank check Destination" where "BooleanColumn" is true
    GroupedTable32 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable32 = Table.SelectRows(GroupedTable32, each [GroupedData][#"Blank check Destination"]{0} = true),
    #"Expanded GroupedData32" = Table.ExpandTableColumn(FilteredTable32, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 32" = Table.AddColumn(#"Expanded GroupedData32", "Error", each "Blank check Destination"),
    #"Removed Columns32" = Table.RemoveColumns(#"Added error type 32",{"GroupedData.BooleanColumn"}),
    //Appending datasets 31 Blank check Destination
    #"Appended31" = Table.Combine({#"Appended30", #"Removed Columns32"}),

    // Group by "Blank check Origin" where "BooleanColumn" is true
    GroupedTable33 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable33 = Table.SelectRows(GroupedTable33, each [GroupedData][#"Blank check Origin"]{0} = true),
    #"Expanded GroupedData33" = Table.ExpandTableColumn(FilteredTable33, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 33" = Table.AddColumn(#"Expanded GroupedData33", "Error", each "Blank check Origin"),
    #"Removed Columns33" = Table.RemoveColumns(#"Added error type 33",{"GroupedData.BooleanColumn"}),
    //Appending datasets 32 Blank check Origin
    #"Appended32" = Table.Combine({#"Appended31", #"Removed Columns33"}),

    // Group by "Blank check Mode" where "BooleanColumn" is true
    GroupedTable34 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable34 = Table.SelectRows(GroupedTable34, each [GroupedData][#"Blank check Mode"]{0} = true),
    #"Expanded GroupedData34" = Table.ExpandTableColumn(FilteredTable34, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 34" = Table.AddColumn(#"Expanded GroupedData34", "Error", each "Blank check Mode"),
    #"Removed Columns34" = Table.RemoveColumns(#"Added error type 34",{"GroupedData.BooleanColumn"}),
    //Appending datasets 33 Blank check Mode
    #"Appended33" = Table.Combine({#"Appended32", #"Removed Columns34"}),

    // Group by "Blank check Trans" where "BooleanColumn" is true
    GroupedTable35 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable35 = Table.SelectRows(GroupedTable35, each [GroupedData][#"Blank check Trans"]{0} = true),
    #"Expanded GroupedData35" = Table.ExpandTableColumn(FilteredTable35, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 35" = Table.AddColumn(#"Expanded GroupedData35", "Error", each "Blank check Trans"),
    #"Removed Columns35" = Table.RemoveColumns(#"Added error type 35",{"GroupedData.BooleanColumn"}),
    //Appending datasets 34 Blank check Trans
    #"Appended34" = Table.Combine({#"Appended33", #"Removed Columns35"}),

    // Group by "Blank check House Ref" where "BooleanColumn" is true
    GroupedTable36 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable36 = Table.SelectRows(GroupedTable36, each [GroupedData][#"Blank check House Ref"]{0} = true),
    #"Expanded GroupedData36" = Table.ExpandTableColumn(FilteredTable36, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 36" = Table.AddColumn(#"Expanded GroupedData36", "Error", each "Blank check House Ref"),
    #"Removed Columns36" = Table.RemoveColumns(#"Added error type 36",{"GroupedData.BooleanColumn"}),
    //Appending datasets 35 Blank check House Ref
    #"Appended35" = Table.Combine({#"Appended34", #"Removed Columns36"}),

    // Group by "Blank check Shipment ID" where "BooleanColumn" is true
    GroupedTable37 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable37 = Table.SelectRows(GroupedTable37, each [GroupedData][#"Blank check Shipment ID"]{0} = true),
    #"Expanded GroupedData37" = Table.ExpandTableColumn(FilteredTable37, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 37" = Table.AddColumn(#"Expanded GroupedData37", "Error", each "Blank check Shipment ID"),
    #"Removed Columns37" = Table.RemoveColumns(#"Added error type 37",{"GroupedData.BooleanColumn"}),
    //Appending datasets 36 Blank check Shipment ID
    #"Appended36" = Table.Combine({#"Appended35", #"Removed Columns37"}),

    //test adding in GSDB check
    // Group by "Lane and GSDB check" where "BooleanColumn" is true
    GroupedTable38 = Table.Group(Source, {"Shipment ID"}, {{"GroupedData", each _, type table [#"Shipment ID"=nullable any, BooleanColumn=nullable logical]}}),
    // Filter out groups where "BooleanColumn" is not true
    FilteredTable38 = Table.SelectRows(GroupedTable38, each [GroupedData][#"Lane and GSDB error"]{0} <> "Lane ID info correct"),
    #"Expanded GroupedData38" = Table.ExpandTableColumn(FilteredTable38, "GroupedData", {"BooleanColumn"}, {"GroupedData.BooleanColumn"}),
    #"Added error type 38" = Table.AddColumn(#"Expanded GroupedData38", "Error", each "Lane and GSDB check"),
    #"Removed Columns38" = Table.RemoveColumns(#"Added error type 38",{"GroupedData.BooleanColumn"}),
    //Appending datasets 37 Lane and GSDB check
    #"Appended37" = Table.Combine({#"Appended36", #"Removed Columns38"}),
    #"Joined job operator back in" = Table.NestedJoin(Appended37, {"Shipment ID"}, #"Job operator join", {"Shipment ID"}, "Job operator join", JoinKind.LeftOuter),
    #"Expanded Job operator join" = Table.ExpandTableColumn(#"Joined job operator back in", "Job operator join", {"Job Operator"}, {"Job operator join.Job Operator"}),
    #"Appened all groupby" = Table.AddColumn(#"Expanded Job operator join", "Count", each 1),
    #"Sorted Rows" = Table.Sort(#"Appened all groupby",{{"Shipment ID", Order.Ascending}})
in
    #"Sorted Rows"

Error output summery sheet
let
    Source = #"Error output sheet",
    #"Grouped Rows" = Table.Group(Source, {"Error"}, {{"Error count", each List.Sum([Count]), type number}})
in
    #"Grouped Rows"

Operator join

let
    Source = #"DSV Shipment Report",
    #"Removed Columns" = Table.RemoveColumns(Source,{"House Ref", "Trans", "Mode", "Origin", "Destination", "Shipment Dest ETA", "Job Branch", "Job Dept", "INCO", "Weight", "UQ", "Volume", "UQ_1", "UQ_2", "Chargeable", "Actual Pickup (Date Only)", "Consignor Name", "Consignor Country", "Destination GSDB Code", "Consignee Name", "Consignee Country", "Carrier Name", "Load(ARV - LEG)", "Disch(ARV - LEG)", "Vessel (ARV - LEG)", "Job Status", "Containers", "Job Opened", "FMC Reference", "Shipment Origin ETD", "ETD Load(ARV - LEG)", "First SEA Leg ATD", "Last Leg ATA", "RefNums (SCN)", "RefNums (INO)", "LANE_ID", "RefNums (SRF)", "Event (GIN - %|FAC=CTO|LOC=USDET%)", "Event (DEP - %|FAC=CTO|LOC=USDET|MOD=RAI%)", "Consol Type", "Carrier_CW1_names.Carrier name", "Carrier_CW1_names.Ocean Carrier", "Carrier_CW1_names.Carrier Code", "Blank check RefNums (SCN)", "Blank check RefNums (INO)", "Blank check RefNums (LAI)", "Blank check RefNums (SRF)", "Blank check Destination GSDB Code", "Blank check First SEA Leg ATD", "Blank check ETD Load(ARV - LEG)", "Blank check Shipment Origin ETD", "Blank check Job Opened", "Blank check Containers", "Blank check FMC Reference", "Blank check Job Status", "Blank check Vessel (ARV - LEG)", "Blank check Disch(ARV - LEG)", "Blank check Load(ARV - LEG)", "Blank check Carrier Name", "Blank check Consignee Country", "Blank check Consignee Name", "Blank check Consignor Country", "Blank check Consignor Name", "Blank check Actual Pickup (Date Only)", "Blank check Chargeable", "Blank check UQ_2", "Blank check UQ_1", "Blank check Volume", "Blank check UQ", "Blank check Weight", "Blank check INCO", "Blank check Job Dept", "Blank check Job Branch", "Blank check Shipment Dest ETA", "Blank check Destination", "Blank check Origin", "Blank check Mode", "Blank check Trans", "Blank check House Ref", "Blank check Shipment ID", "Lane_check_table.LANE ID", "Lane_check_table.Origin_GSDB_check", "Lane_check_table.Destination_GSDB_check", "Lane_check_table.Carrier_check", "Lane and GSDB check"})
in
    #"Removed Columns"

combinations CW1 Combinations_CW1_codes
let
    Source = Excel.CurrentWorkbook(){[Name="Combinations_CW1_codes"]}[Content],
    #"Changed Type" = Table.TransformColumnTypes(Source,{{"Consignor Name", type text}, {"Consignee Name", type text}, {"Destination Code", type text}, {"Lane ID", type text}, {"App B  Dest GSDB", type text}}),
    #"Added Custom" = Table.AddColumn(#"Changed Type", "Merge_column_key", each [Destination Code] & [Lane ID]),
    #"Removed Columns" = Table.RemoveColumns(#"Added Custom",{"Merge_column_key"}),
    #"Added merge_key" = Table.AddColumn(#"Removed Columns", "Merge_column_key", each [Destination Code] & " " & [Lane ID]),
    #"Changed merge_key" = Table.TransformColumnTypes(#"Added merge_key",{{"Merge_column_key", type text}}),
    #"Added validation column" = Table.AddColumn(#"Changed merge_key", "Dest_GSDB_valid", each true)
in
    #"Added validation column"

Lane ID check

let
    Source = #"DSV Shipment Report",
    #"Filtered GSDB errors" = Table.SelectRows(Source, each ([Lane and GSDB check] = "Destination GSDB error")),
    #"Merged Queries" = Table.NestedJoin(#"Filtered GSDB errors", {"Lane_check_table.LANE ID"}, Combinations_CW1_codes, {"Lane ID"}, "Combinations_CW1_codes", JoinKind.LeftOuter),
    #"Expanded Combinations_CW1_codes" = Table.ExpandTableColumn(#"Merged Queries", "Combinations_CW1_codes", {"Lane ID"}, {"Combinations_CW1_codes.Lane ID"})
in
    #"Expanded Combinations_CW1_codes"