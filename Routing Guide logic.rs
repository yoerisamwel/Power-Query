Routing Guide logic

let
    Source = Excel.CurrentWorkbook(){[Name="Table3"]}[Content],
    #"Filtered Rows" = Table.SelectRows(Source, each [Changes] = "Please ADD NEW"),
    #"Changed Type" = Table.TransformColumnTypes(#"Filtered Rows",{{"Changes", type text}, {"MODE", Int64.Type}, {"COMMODITY", type text}, {"SUPPLIER GSDB", type text}, {"SUPPLIER CMF", type text}, {"PLANT GSDB", type text}, {"PLANT CMF", type text}, 
    {"SHIP FREQUENCY", Int64.Type}, {"1 DIGIT DAY", Int64.Type}, {"DIRECTION", type text}, {"REGION", type text}, {"Region Code", type text}, {"Country Code", Int64.Type}, {"FORD ROUTE ID", type text}, {"TOTAL DAYS CMMS", Int64.Type}, {"FLOAT (CMMS - ROUTE GUIDE)", 
    Int64.Type}, {"TOTAL DAYS ROUTE GUIDE", Int64.Type}, {"DAYS SFS TO AOP (FCL)", type text}, {"DAYS SFS TO ACC (LCL)", Int64.Type}, {"DAYS ACC TO DCC (LCL)", Int64.Type}, {"DAYS DCC TO AOP (LCL)", Int64.Type}, {"SUPPLIER", type text}, {"SUPPLIER BUSINESS NAME", type text}, 
    {"STREET", type text}, {"CITY", type text}, {"COUNTRY", type text}, {"POSTAL CODE", type any}, {"CONTACT NAME", type any}, {"PHONE NUMBER", type any}, {"EMAIL ADDRESS 1", type any}, {"EMAIL ADDRESS 2", type any}, {"EMAIL ADDRESS 3", Int64.Type}, 
    {"SSL PICK UP LOC", type text}, {"SSL PICK UP LOC NAME", type text}, {"SSL PICK UP LOC ADDRESS", type text}, {"SSL PICK UP LOC CITY", type text}, {"SSL PICK UP LOC ST, PR, TE", type text}, {"SSL PICK UP COUNTRY", type text}, {"SSL PICKUP POSTAL CODE", Int64.Type}, 
    {"SSL PICK UP LOAD TYPE", type text}, {"Container_Type_20", type text}, {"Container_Type_40    ", type text}, {"Container_Type_40HC    ", type text}, {"Container_Type_45    ", type text}, {"CONSOLIDATOR", type text}, {"CONSOLIDATOR GSDB", type text}, 
    {"Carrier#(lf)(Direct / Pre-#(lf)ODC/CC#(lf)collection)", type text}, {"CARRIER CONTACT SURNAME", type text}, {"CARRIER CONTACT NAME", type text}, {"CARRIER CONTACT LOCATION", type text}, {"CARRIER PHONE", type text}, {"CARRIER FAX", type text}, 
    {"CARRIER E-MAIL", type text}, {"CARRIER MOBILE", type text}, {"1st#(lf)ODC/CC", type text}, {"LTL#(lf)Carrier#(lf)ex 1st ODC/CC", type text}, {"2nd#(lf)ODC/CC", type text}, {"LTL#(lf)Carrier#(lf)ex2nd ODC/CC", type text}, 
    {"SUP TO CONSOL CARRIER SCAC", type text}, {"PRE CONSOL STOP", type any}, {"CONSOLIDATOR (POOL) CUT-OFF", type text}, {"CONSOLIDATOR (POOL) CUT-OFF time", Int64.Type}, {"MODE TO PORT", type text}, {"LOAD YARD LOCATION", type any}, 
    {"LOAD YARD CUT-OFF", type text}, {"PORT CUT OFF", type text}, {"ORIGIN PORT", type text}, {"NEW PORT CODE", type text}, {"DAYS AOP TO OTB", Int64.Type}, {"DAYS OTB TO ADP", Int64.Type}, {"ESTIMATED SA DAY", type text}, 
    {"ESTIMATED AR DAY", type text}, {"SSL NAME", type text}, {"SSL SCAC", type text}, {"% OF BUSINESS WITH SSL", Int64.Type}, {"SSL GSDB", type text}, {"SSL CONTRACT NUMBER", type text}, {"INTERMITTENT PORT", type text}, 
    {"DESTINATION PORT", type text}, {"NEW PORT CODE2", type text}, {"YARD LOCATION UNLOAD", Int64.Type}, {"MODE FROM PORT", type text}, {"DAYS ADP TO TLD", Int64.Type}, {"SSL DROP OFF LOC", type text}, {"SSL DROP OFF LOCATION NAME", type text}, 
    {"SSL DROP OFF LOC ADDRESS", type text}, {"SSL DROP OFF LOC CITY", type text}, {"SSL DROP OF LOC ST, PR, TR", type text}, {"SSL DROP OFF LOC CON CODE", type text}, {"SSL DROP OFF LOC POSTAL CODE", Int64.Type}, {"SSL DROP OFF UNLOAD TYPE", type text}, 
    {"DECONSOLIDATOR", type text}, {"DECON TO PLT CARRIER SCAC", type any}, {"POST DECONSOL STOP", type any}, {"DAYS TLD TO ADC (LCL)", type any}, {"DAYS ADC TO DDC (LCL)", type any}, {"DAYS DDC TO DPL (LCL)", type any}, {"DAYS TLD TO DPL (FCL)", type any}, 
    {"ALTCONSIGNEE", type text}, {"PLANT", type text}, {"PLANT BUSINESS NAME", type text}, {"PLANT STREET NAME", type text}, {"CITY3", type text}, {"STATE, PROV, TERR", type text}, {"COUNTRY4", type text}, {"POSTAL CODE5", type text}, {"DOCK CODE", type any}, 
    {"ADDITIONAL CMMS EXCEPTION TRANSIT", Int64.Type}, {"ADDITIONAL RG EXCEPTION TRANSIT", Int64.Type}, {"EXCEPTION TRANSIT REASON", type any}, {"CMMS TT W/O EXCEPTION TRANSIT", type any}, {"RG TT W/O EXCEPTION TRANSIT ", Int64.Type}, 
    {"START EFFECTIVE YEAR", Int64.Type}, {"START EFFECTIVE MONTH", Int64.Type}, {"START EFFECTIVE DAY", Int64.Type}, {"END EFFECTIVE YEAR", Int64.Type}, {"END EFFECTIVE MONTH", Int64.Type}, {"END EFFECTIVE DAY", Int64.Type}, {"CUSTOMS BROKER", type text}, 
    {"PAYMENT TERMS", type text}, {"TDI MODE", type text}, {"ACTIVE/INACTIVE", type text}, {"PRIORITY", Int64.Type}, {"ROUTING_DESCRIPTOR ", Int64.Type}, {"NAME OF CARRIER'S SERVICE", type text}}),
    #"Removed pulled columns" = Table.RemoveColumns(#"Changed Type",{"SUPPLIER CMF", "PLANT CMF", "SUPPLIER BUSINESS NAME", "STREET", "CITY", "COUNTRY", "POSTAL CODE", "SSL PICK UP LOC NAME", 
    "SSL PICK UP LOC ADDRESS", "SSL PICK UP LOC CITY", "SSL PICK UP LOC ST, PR, TE", "SSL PICK UP COUNTRY", "SSL PICKUP POSTAL CODE", "SSL DROP OFF LOCATION NAME", "SSL DROP OFF LOC ADDRESS", 
    "SSL DROP OFF LOC CITY", "SSL DROP OF LOC ST, PR, TR", "SSL DROP OFF LOC CON CODE", "SSL DROP OFF LOC POSTAL CODE", "PLANT", "PLANT BUSINESS NAME", "PLANT STREET NAME", "CITY3", 
    "STATE, PROV, TERR", "COUNTRY4", "POSTAL CODE5", "INTERMITTENT PORT", "DESTINATION PORT", "SSL NAME", "PAYMENT TERMS", "ORIGIN PORT", "ESTIMATED SA DAY", "ESTIMATED AR DAY", "TOTAL DAYS ROUTE GUIDE", 
    "CMMS TT W/O EXCEPTION TRANSIT", "EXCEPTION TRANSIT REASON", "RG TT W/O EXCEPTION TRANSIT ", "NEW PORT CODE2", "SSL SCAC", "SSL CONTRACT NUMBER", "NEW PORT CODE", "Changes", "Container_Type_20", 
    "Container_Type_40    ", "Container_Type_40HC    ", "SUPPLIER", "ADDITIONAL CMMS EXCEPTION TRANSIT", "FLOAT (CMMS - ROUTE GUIDE)", "ROUTING_DESCRIPTOR ", "NAME OF CARRIER'S SERVICE"}),
    //Added Key column to be used in joins
    #"Added 2346" = Table.AddColumn(#"Removed pulled columns", "2346", each [FORD ROUTE ID]&[SSL PICK UP LOC]&[SSL DROP OFF LOC]&[SSL GSDB]),
    //merged in the supplier from the CMF table
    #"Merged supplier cmf pull" = Table.NestedJoin(#"Added 2346", {"SUPPLIER GSDB"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded supplier cmf pull" = Table.ExpandTableColumn(#"Merged supplier cmf pull", "CMF", {"uTrac Name"}, {"CMF.uTrac Name"}),
    #"Renamed supplier cmf column" = Table.RenameColumns(#"Expanded supplier cmf pull",{{"CMF.uTrac Name", "Supplier cmf"}}),
    //Collected plant information from CMF table
    #"Merged Plant cmf" = Table.NestedJoin(#"Renamed supplier cmf column", {"PLANT GSDB"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded Plant cmf" = Table.ExpandTableColumn(#"Merged Plant cmf", "CMF", {"uTrac Name"}, {"CMF.uTrac Name"}),
    #"Renamed Columns" = Table.RenameColumns(#"Expanded Plant cmf",{{"CMF.uTrac Name", "Plant cmf"}}),
    #"Added SUPPLIER column" = Table.AddColumn(#"Renamed Columns", "SUPPLIER", each [SUPPLIER GSDB]),
    //collected supplier information from the CMF table
    #"Merged SUPPLIER columns" = Table.NestedJoin(#"Added SUPPLIER column", {"SUPPLIER GSDB"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded SUPPLIER columns" = Table.ExpandTableColumn(#"Merged SUPPLIER columns", "CMF", {"Business Name", "Street", "City", "Country", "Postal/ZIP"}, {"CMF.Business Name", "CMF.Street", "CMF.City", "CMF.Country", "CMF.Postal/ZIP"}),
    #"Renamed SUPPLIER columns" = Table.RenameColumns(#"Expanded SUPPLIER columns",{{"CMF.Business Name", "SUPPLIER BUSINESS NAME"}, {"CMF.Street", "STREET"}, {"CMF.City", "CITY"}, {"CMF.Country", "COUNTRY"}, {"CMF.Postal/ZIP", "POSTAL CODE"}}),
    //collected ssl pickup information from the CMF table
    #"Merged ssl pickup" = Table.NestedJoin(#"Renamed SUPPLIER columns", {"SSL PICK UP LOC"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded ssl pickup" = Table.ExpandTableColumn(#"Merged ssl pickup", "CMF", {"uTrac Name", "Street", "City", "State/Prov", "Country", "Postal/ZIP"}, {"CMF.uTrac Name", "CMF.Street", "CMF.City", "CMF.State/Prov", "CMF.Country", "CMF.Postal/ZIP"}),
    #"Renamed ssl pickup" = Table.RenameColumns(#"Expanded ssl pickup",{{"CMF.uTrac Name", "SSL PICK UP LOC NAME"}, {"CMF.Street", "SSL PICK UP LOC ADDRESS"}, {"CMF.City", "SSL PICK UP LOC CITY"}, {"CMF.State/Prov", 
    "SSL PICK UP LOC ST, PR, TE"}, {"CMF.Country", "SSL PICK UP COUNTRY"}, {"CMF.Postal/ZIP", "SSL PICKUP POSTAL CODE"}}),
    //collected the ssl pickup information from the CMF table
    #"Merged ssl dropoff" = Table.NestedJoin(#"Renamed ssl pickup", {"SSL DROP OFF LOC"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded ssl dropoff" = Table.ExpandTableColumn(#"Merged ssl dropoff", "CMF", {"uTrac Name", "Street", "City", "State/Prov", "Country", "Postal/ZIP"}, {"CMF.uTrac Name", "CMF.Street", "CMF.City", "CMF.State/Prov", "CMF.Country", "CMF.Postal/ZIP"}),
    #"Renamed ssl dropoff" = Table.RenameColumns(#"Expanded ssl dropoff",{{"CMF.uTrac Name", "SSL DROP OFF LOCATION NAME"}, {"CMF.Street", "SSL DROP OFF LOC ADDRESS"}, {"CMF.City", "SSL DROP OFF LOC CITY"}, 
    {"CMF.State/Prov", "SSL DROP OF LOC ST, PR, TR"}, {"CMF.Country", "SSL DROP OFF LOC CON CODE"}, {"CMF.Postal/ZIP", "SSL DROP OFF LOC POSTAL CODE"}}),
    #"Added PLANT" = Table.AddColumn(#"Renamed ssl dropoff", "PLANT", each [PLANT GSDB]),
    //collected the plant information from the CMF table
    #"Merged PLANT" = Table.NestedJoin(#"Added PLANT", {"PLANT"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded PLANT" = Table.ExpandTableColumn(#"Merged PLANT", "CMF", {"Business Name", "Street", "City", "State/Prov", "Country", "Postal/ZIP"}, {"CMF.Business Name", "CMF.Street", "CMF.City", "CMF.State/Prov", "CMF.Country", "CMF.Postal/ZIP"}),
    #"Renamed PLANT Columns" = Table.RenameColumns(#"Expanded PLANT",{{"CMF.Business Name", "PLANT BUSINESS NAME"}, {"CMF.Street", "PLANT STREET NAME"}, {"CMF.City", "PLANT CITY"}, 
    {"CMF.State/Prov", "PLANT STATE PROV TERR"}, {"CMF.Country", "PLANT COUNTRY"}, {"CMF.Postal/ZIP", "PLANT POSTAL ZIP"}}),
    //collected additional plant information from the CMF table
    #"Merged State/Prov/terr" = Table.NestedJoin(#"Renamed PLANT Columns", {"SUPPLIER GSDB"}, CMF, {"GSDB"}, "CMF", JoinKind.LeftOuter),
    #"Expanded State/Prov/terr" = Table.ExpandTableColumn(#"Merged State/Prov/terr", "CMF", {"State/Prov"}, {"CMF.State/Prov"}),
    #"Renamed State/Prov/terr Columns" = Table.RenameColumns(#"Expanded State/Prov/terr",{{"CMF.State/Prov", "State/Prov/terr"}}),
    //collected the carrier information from the SSL table
    #"Merged container type columns" = Table.NestedJoin(#"Renamed State/Prov/terr Columns", {"2346"}, SSL, {"LANE ID"}, "SSL", JoinKind.LeftOuter),
    #"Expanded container type columns" = Table.ExpandTableColumn(#"Merged container type columns", "SSL", {"CARRIER", "PAY TERMS (PREPAID, COLLECT OR BOTH)", "EST ANNUAL 20' Volume", "EST ANNUAL 40' Volume", 
    "EST ANNUAL 40' HC Volume", "NAME OF CARRIER'S SERVICE", "EXIT PORT", "TRANS LOAD PORT", "ENTRY PORT", "SAIL DAY", "ARRIVAL DAY", "PORT TO PORT"}, {"SSL.CARRIER", "SSL.PAY TERMS (PREPAID, COLLECT OR BOTH)", 
    "SSL.EST ANNUAL 20' Volume", "SSL.EST ANNUAL 40' Volume", "SSL.EST ANNUAL 40' HC Volume", "SSL.NAME OF CARRIER'S SERVICE", "SSL.EXIT PORT", "SSL.TRANS LOAD PORT", "SSL.ENTRY PORT", "SSL.SAIL DAY", "SSL.ARRIVAL DAY", "SSL.PORT TO PORT"}),
    //reworked logic check if estimated lane volume is 0
    #"Added Container type_20" = Table.AddColumn(#"Expanded container type columns", "Container type_20", each if [#"SSL.EST ANNUAL 20' Volume"] = 0 then "N" else "Y"),
    #"Added Container type_40" = Table.AddColumn(#"Added Container type_20", "Container type_40", each if [#"SSL.EST ANNUAL 40' Volume"] = 0 then "N" else "Y"),
    #"Added Container type_40HC" = Table.AddColumn(#"Added Container type_40", "Container type_40HC", each if [#"SSL.EST ANNUAL 40' HC Volume"] = 0 then "N" else "Y"),
    #"Removed Columns container type columns" = Table.RemoveColumns(#"Added Container type_40HC",{"SSL.EST ANNUAL 20' Volume", "SSL.EST ANNUAL 40' Volume", "SSL.EST ANNUAL 40' HC Volume"}),
    #"Renamed SSL merged columns" = Table.RenameColumns(#"Removed Columns container type columns",{{"SSL.CARRIER", "SSL NAME"}, {"SSL.PAY TERMS (PREPAID, COLLECT OR BOTH)", "SSL.PAYMENT TERMS"}, {"SSL.NAME OF CARRIER'S SERVICE", 
    "NAME OF CARRIER'S SERVICE"}, {"SSL.EXIT PORT", "ORIGIN PORT"}, {"SSL.TRANS LOAD PORT", "INTERMITTEN PORT"}, {"SSL.ENTRY PORT", "DESTINATION PORT"}, {"SSL.SAIL DAY", "ESTIMATED SA DAY"}, {"SSL.ARRIVAL DAY", "ESTIMATED AR DAY"}, {"SSL.PORT TO PORT", "PORT TO PORT"}}),
    //recreated simple excel logic for these columns
    #"Added Routing_Descriptor" = Table.AddColumn(#"Renamed SSL merged columns", "Routing_Descriptor", each [PRIORITY]),
    #"Added TOTAL DAYS ROUTE GUIDE" = Table.AddColumn(#"Added Routing_Descriptor", "TOTAL DAYS ROUTE GUIDE", each if [MODE] = 24 then [#"DAYS SFS TO ACC (LCL)"] + [#"DAYS ACC TO DCC (LCL)"] + [#"DAYS DCC TO AOP (LCL)"] + [DAYS AOP TO OTB] + [DAYS OTB TO ADP] + 
    [DAYS ADP TO TLD] + [#"DAYS TLD TO DPL (FCL)"] else if [MODE] = 23 then [#"DAYS SFS TO ACC (LCL)"] + [#"DAYS ACC TO DCC (LCL)"] + [#"DAYS DCC TO AOP (LCL)"] + [DAYS AOP TO OTB] + [DAYS OTB TO ADP] + [DAYS ADP TO TLD] + [#"DAYS TLD TO ADC (LCL)"] +
     [#"DAYS ADC TO DDC (LCL)"] + [#"DAYS DDC TO DPL (LCL)"] else if [MODE] = 22 then [#"DAYS SFS TO AOP (FCL)"] + [DAYS AOP TO OTB] + [DAYS OTB TO ADP] + [DAYS ADP TO TLD] + [#"DAYS TLD TO ADC (LCL)"] + [#"DAYS ADC TO DDC (LCL)"] + [#"DAYS DDC TO DPL (LCL)"] 
     else if [MODE] = 21 then [#"DAYS SFS TO AOP (FCL)"] + [DAYS AOP TO OTB] + [DAYS OTB TO ADP] + [DAYS ADP TO TLD] + [#"DAYS TLD TO DPL (FCL)"] else "error"),
    #"Added FLOAT (CMMS-ROUTE GUIDE)" = Table.AddColumn(#"Added TOTAL DAYS ROUTE GUIDE", "FLOAT (CMMS-ROUTE GUIDE)", each if [COMMODITY] = "AUTO" then [TOTAL DAYS CMMS] - [TOTAL DAYS ROUTE GUIDE] else "RACK"),
    #"Added MARKUS_REPORT_JOIN_KEY" = Table.AddColumn(#"Added FLOAT (CMMS-ROUTE GUIDE)", "MARKUS_REPORT_JOIN_KEY", each [SUPPLIER GSDB] & [PLANT GSDB]),
    //collected information from the markus report
    #"Merged Queries" = Table.NestedJoin(#"Added MARKUS_REPORT_JOIN_KEY", {"MARKUS_REPORT_JOIN_KEY"}, #"Markus Report", {"ID"}, "Markus Report", JoinKind.LeftOuter),
    #"Expanded Markus Report" = Table.ExpandTableColumn(#"Merged Queries", "Markus Report", {"Transit Days", "Transit Overide Remarks", "Ship Frequency 31/32 Day", "TT override", "SF overrrid"}, {"Markus Report.Transit Days", 
    "Markus Report.Transit Overide Remarks", "Markus Report.Ship Frequency 31/32 Day", "Markus Report.TT override", "Markus Report.SF overrrid"}),
    #"Renamed Markus report join" = Table.RenameColumns(#"Expanded Markus Report",{{"Markus Report.Transit Days", "CMMS TT W/O EXCEPTION TRANSIT"}, {"Markus Report.Transit Overide Remarks", "EXCEPTION TRANSIT REASON"}, 
    {"Markus Report.Ship Frequency 31/32 Day", "MARKUS DIGIT DAY"}, {"Markus Report.TT override", "MARKUS CMMS TIME"}, {"Markus Report.SF overrrid", "MARKUS SF"}}),
    #"Trimmed Text Markus report Join" = Table.TransformColumns(Table.TransformColumnTypes(#"Renamed Markus report join", {{"CMMS TT W/O EXCEPTION TRANSIT", type text}, {"MARKUS DIGIT DAY", type text}, {"MARKUS CMMS TIME", type text}, 
    {"MARKUS SF", type text}}, "en-US"),{{"CMMS TT W/O EXCEPTION TRANSIT", Text.Trim, type text}, {"EXCEPTION TRANSIT REASON", Text.Trim, type text}, {"MARKUS DIGIT DAY", Text.Trim, type text}, {"MARKUS CMMS TIME", Text.Trim, type text}, 
    {"MARKUS SF", Text.Trim, type text}}),
    #"Added TOTAL DAYS CMMS (TRIM)" = Table.AddColumn(#"Trimmed Text Markus report Join", "TOTAL DAYS CMMS (TRIM)", each [TOTAL DAYS CMMS]),
    #"Added SHIP FREQUENCY" = Table.AddColumn(#"Added TOTAL DAYS CMMS (TRIM)", "SHIP FREQUENCY (TRIM)", each [SHIP FREQUENCY]),
    #"Added 1 DIGIT DAY (TRIM)" = Table.AddColumn(#"Added SHIP FREQUENCY", "1 DIGIT DAY (TRIM)", each [1 DIGIT DAY]),
    #"Trimmed Text" = Table.TransformColumns(Table.TransformColumnTypes(#"Added 1 DIGIT DAY (TRIM)", {{"TOTAL DAYS CMMS (TRIM)", type text}, {"SHIP FREQUENCY (TRIM)", type text}, {"1 DIGIT DAY (TRIM)", type text}}, "en-US"),
    {{"TOTAL DAYS CMMS (TRIM)", Text.Trim, type text}, {"SHIP FREQUENCY (TRIM)", Text.Trim, type text}, {"1 DIGIT DAY (TRIM)", Text.Trim, type text}}),
    #"Changed Type1" = Table.TransformColumnTypes(#"Trimmed Text",{{"TOTAL DAYS CMMS (TRIM)", Int64.Type}, {"CMMS TT W/O EXCEPTION TRANSIT", Int64.Type}}),
    #"Added ADDITIONAL CMMS EXCEPTION TRANSIT" = Table.AddColumn(#"Changed Type1", "ADDITIONAL CMMS EXCEPTION TRANSIT", each if [COMMODITY] = "AUTO" then [TOTAL DAYS CMMS] - [#"CMMS TT W/O EXCEPTION TRANSIT"] else "0"),
    #"Added RG TT W/O EXCEPTION TRANSIT" = Table.AddColumn(#"Added ADDITIONAL CMMS EXCEPTION TRANSIT", "RG TT W/O EXCEPTION TRANSIT", each [TOTAL DAYS ROUTE GUIDE] - [ADDITIONAL RG EXCEPTION TRANSIT]),
    //merged in the port information from the PORT table
    #"Merged Ports" = Table.NestedJoin(#"Added RG TT W/O EXCEPTION TRANSIT", {"ORIGIN PORT"}, Ports, {"Port name"}, "Ports", JoinKind.LeftOuter),
    #"Expanded Ports" = Table.ExpandTableColumn(#"Merged Ports", "Ports", {"Port Digit Code"}, {"Ports.Port Digit Code"}),
    #"Renamed NEW PORT CODE" = Table.RenameColumns(#"Expanded Ports",{{"Ports.Port Digit Code", "Ports.NEW PORT CODE"}}),
    #"Merged SSL SCAC" = Table.NestedJoin(#"Renamed NEW PORT CODE", {"SSL NAME"}, Carriers, {"Carrier name"}, "Carriers", JoinKind.LeftOuter),
    #"Expanded Carriers" = Table.ExpandTableColumn(#"Merged SSL SCAC", "Carriers", {"Carrier SCAC"}, {"Carriers.Carrier SCAC"}),
    #"Renamed SSL SCAC" = Table.RenameColumns(#"Expanded Carriers",{{"Carriers.Carrier SCAC", "SSL SCAC"}}),
    #"Merged SSL CONTRACT NUMBER" = Table.NestedJoin(#"Renamed SSL SCAC", {"SSL SCAC"}, Carriers, {"Carrier SCAC"}, "Carriers", JoinKind.LeftOuter),
    #"Expanded SSL CONTRACT NUMBER" = Table.ExpandTableColumn(#"Merged SSL CONTRACT NUMBER", "Carriers", {"Contract Number"}, {"Carriers.Contract Number"}),
    #"Merged NEW PORT CODE" = Table.NestedJoin(#"Expanded SSL CONTRACT NUMBER", {"DESTINATION PORT"}, Ports, {"Port name"}, "Ports", JoinKind.LeftOuter),
    #"Expanded NEW PORT CODE" = Table.ExpandTableColumn(#"Merged NEW PORT CODE", "Ports", {"Port Digit Code"}, {"Ports.Port Digit Code"}),
    #"Renamed Columns1" = Table.RenameColumns(#"Expanded NEW PORT CODE",{{"Ports.Port Digit Code", "NEW PORT CODE"}, {"Carriers.Contract Number", "SSL CONTRACT NUMBER"}}),
    Removed_markuskey_column = Table.RemoveColumns(#"Renamed Columns1",{"MARKUS_REPORT_JOIN_KEY"}),
    //reordered columns to match old mastet file sheet
    #"Reordered Columns" = Table.ReorderColumns(Removed_markuskey_column,{"2346", "MODE", "COMMODITY", "SUPPLIER GSDB","Supplier cmf", "PLANT GSDB", "Plant cmf", "SHIP FREQUENCY", "1 DIGIT DAY", "DIRECTION", "REGION", "Region Code", "Country Code","FORD ROUTE ID", "TOTAL DAYS CMMS",
    "FLOAT (CMMS-ROUTE GUIDE)",  "TOTAL DAYS ROUTE GUIDE", "DAYS SFS TO AOP (FCL)", "DAYS SFS TO ACC (LCL)", "DAYS ACC TO DCC (LCL)", "DAYS DCC TO AOP (LCL)", "SUPPLIER", "SUPPLIER BUSINESS NAME", "STREET", "CITY", "COUNTRY", "POSTAL CODE",
    "CONTACT NAME", "PHONE NUMBER", "EMAIL ADDRESS 1", "EMAIL ADDRESS 2", "EMAIL ADDRESS 3", "SSL PICK UP LOC", "SSL PICK UP LOC NAME", "SSL PICK UP LOC ADDRESS", "SSL PICK UP LOC CITY", "SSL PICK UP LOC ST, PR, TE", "SSL PICK UP COUNTRY", "SSL PICKUP POSTAL CODE",
    "SSL PICK UP LOAD TYPE","Container type_20", "Container type_40", "Container type_40HC", "Container_Type_45    ",  "CONSOLIDATOR", "CONSOLIDATOR GSDB","Carrier#(lf)(Direct / Pre-#(lf)ODC/CC#(lf)collection)", 
    "CARRIER CONTACT SURNAME", "CARRIER CONTACT NAME", "CARRIER CONTACT LOCATION", "CARRIER PHONE", "CARRIER FAX", "CARRIER E-MAIL", "CARRIER MOBILE", "1st#(lf)ODC/CC", "LTL#(lf)Carrier#(lf)ex 1st ODC/CC", "2nd#(lf)ODC/CC", "LTL#(lf)Carrier#(lf)ex2nd ODC/CC",   
    "SUP TO CONSOL CARRIER SCAC", "PRE CONSOL STOP", "CONSOLIDATOR (POOL) CUT-OFF", "CONSOLIDATOR (POOL) CUT-OFF time", "MODE TO PORT", "LOAD YARD LOCATION", "LOAD YARD CUT-OFF", "PORT CUT OFF", "ORIGIN PORT", "Ports.NEW PORT CODE","DAYS AOP TO OTB", "DAYS OTB TO ADP",   
    "ESTIMATED SA DAY", "ESTIMATED AR DAY", "SSL NAME", "SSL SCAC", "% OF BUSINESS WITH SSL", "SSL GSDB", "SSL CONTRACT NUMBER","INTERMITTEN PORT",  "DESTINATION PORT", "NEW PORT CODE",
    "YARD LOCATION UNLOAD", "MODE FROM PORT", "DAYS ADP TO TLD", "SSL DROP OFF LOC", "SSL DROP OFF LOCATION NAME","SSL DROP OFF LOC ADDRESS",  "SSL DROP OFF LOC CITY", "SSL DROP OF LOC ST, PR, TR", "SSL DROP OFF LOC CON CODE", "SSL DROP OFF LOC POSTAL CODE",
    "SSL DROP OFF UNLOAD TYPE", "DECONSOLIDATOR", "DECON TO PLT CARRIER SCAC", "POST DECONSOL STOP", "DAYS TLD TO ADC (LCL)", "DAYS ADC TO DDC (LCL)", "DAYS DDC TO DPL (LCL)", "DAYS TLD TO DPL (FCL)",  "ALTCONSIGNEE", "PLANT", "PLANT BUSINESS NAME",
    "PLANT STREET NAME", "PLANT CITY", "PLANT STATE PROV TERR", "PLANT COUNTRY", "PLANT POSTAL ZIP", "DOCK CODE", "ADDITIONAL CMMS EXCEPTION TRANSIT", "ADDITIONAL RG EXCEPTION TRANSIT", "EXCEPTION TRANSIT REASON", "CMMS TT W/O EXCEPTION TRANSIT",
    "RG TT W/O EXCEPTION TRANSIT", "START EFFECTIVE YEAR", "START EFFECTIVE MONTH", "START EFFECTIVE DAY", "END EFFECTIVE YEAR", "END EFFECTIVE MONTH", "END EFFECTIVE DAY",
    "CUSTOMS BROKER",  "SSL.PAYMENT TERMS",  "TDI MODE", "ACTIVE/INACTIVE", "PRIORITY", "Routing_Descriptor",
    //   UPDATE FLAG FOR SHIPPING INSTRUCTIONS
    //   EMAIL ADDRESS 4    
    "State/Prov/terr", "NAME OF CARRIER'S SERVICE","MARKUS CMMS TIME", "TOTAL DAYS CMMS (TRIM)", "MARKUS SF", "SHIP FREQUENCY (TRIM)","MARKUS DIGIT DAY", "1 DIGIT DAY (TRIM)"})
in
    #"Reordered Columns"

milestone_feed tab
let
    Source = master_old_data,
    #"Changed Type" = Table.TransformColumnTypes(Source,{{"2346", type text}}),
    #"Renamed Columns" = Table.RenameColumns(#"Changed Type",{{"MODE", "mmode"}, {"SUPPLIER GSDB", "supplier_cd"}, {"PLANT GSDB", "plant_cd"}, {"COMMODITY", "commodity"}, {"SHIP FREQUENCY", "ship_freq,"}, {"FORD ROUTE ID", "route_id,"}, {"TOTAL DAYS ROUTE GUIDE", "total_transit,"}, {"DAYS AOP TO OTB", "aop_otb,"}, {"DAYS OTB TO ADP", "otb_adp,"}, {"DAYS ADP TO TLD", "adp_tld,"}, {"START EFFECTIVE YEAR", "trash1"}, {"START EFFECTIVE MONTH", "trash2"}, {"START EFFECTIVE DAY", "trash3"}, {"END EFFECTIVE YEAR", "trash4"}, {"END EFFECTIVE MONTH", "trash5"}, {"END EFFECTIVE DAY", "trash6"}, {"TDI MODE", "Mode"}, {"DIRECTION", "Direction"}, {"ADDITIONAL CMMS EXCEPTION TRANSIT", "Exception_Total"}, {"EXCEPTION TRANSIT REASON", "Exception_Reason"}, {"MODE TO PORT", "CarrierA_SFS_Mode"}, {"CONSOLIDATOR GSDB", "Consol_cd"}, {"SSL GSDB", "SSL_GSDB6"}, {"SSL SCAC", "CarrierA_TLD_SCAC"}, {"SSL CONTRACT NUMBER", "Contract_Number"}, {"% OF BUSINESS WITH SSL", "SSL_Pickup_cd"}, {"SSL PICK UP LOAD TYPE", "SSL_Pickup_Type"}, {"MODE FROM PORT", "CarrierA_TLD_Mode"}, {"SSL DROP OFF UNLOAD TYPE", "Delivery_Type"}, {"SSL DROP OFF LOC", "SSL_DropOff_GSDB"}, {"PRIORITY", "Priority"}, {"TOTAL DAYS CMMS", "cmms_transit,"}, {"NEW PORT CODE", "Origin_Port"}}),
    //rounded off the float int he column
    #"Rounded Off cmms_transit," = Table.TransformColumns(#"Renamed Columns",{{"cmms_transit,", each Number.Round(_, 0), Int64.Type}}),
    // merged in the code sheet
    #"Merged Queries" = Table.NestedJoin(#"Rounded Off cmms_transit,", {"mmode"}, Codes, {"Code"}, "Codes", JoinKind.LeftOuter),
    #"Expanded Codes" = Table.ExpandTableColumn(#"Merged Queries", "Codes", {"Export", "Import"}, {"Codes.Export", "Codes.Import"}),
    #"Renamed load_type columns" = Table.RenameColumns(#"Expanded Codes",{{"Codes.Export", "Origin_Load_Type"}, {"Codes.Import", "Destn_Load_Type"}}),
    //rewrote the excel formulas used in the routingguide
    #"Added sfs_acc," = Table.AddColumn(#"Renamed load_type columns", "sfs_acc,", each if [Origin_Load_Type] = "LCL" then [#"DAYS SFS TO ACC (LCL)"] else 999),
    #"Added acc_dcc," = Table.AddColumn(#"Added sfs_acc,", "acc_dcc,", each if [Origin_Load_Type] = "LCL" then [#"DAYS ACC TO DCC (LCL)"] else 999),
    #"Added tld_adc," = Table.AddColumn(#"Added acc_dcc,", "tld_adc,", each if [Destn_Load_Type] = "LCL" then [#"DAYS TLD TO ADC (LCL)"] else "999"),
    #"Added adc_ddc," = Table.AddColumn(#"Added tld_adc,", "adc_ddc,", each if [Destn_Load_Type] = "LCL" then [#"DAYS ADC TO DDC (LCL)"] else "999"),
    #"Added dcc_aop," = Table.AddColumn(#"Added adc_ddc,", "dcc_aop,", each if [Origin_Load_Type] = "LCL" then [#"DAYS DCC TO AOP (LCL)"] else [#"DAYS SFS TO AOP (FCL)"]),
    #"Added ddc_dpl," = Table.AddColumn(#"Added dcc_aop,", "ddc_dpl,", each if [Destn_Load_Type] = "LCL" then [#"DAYS DDC TO DPL (LCL)"] else [#"DAYS TLD TO DPL (FCL)"]),
    //used nested if statements to recreate an excel or operator
    #"Added DeConsol_cd" = Table.AddColumn(#"Added ddc_dpl,", "DeConsol_cd", each if [DECONSOLIDATOR] = " " then " " else if [DECONSOLIDATOR] = "FCL" then " " else if [DECONSOLIDATOR] = 0 then " " else if [DECONSOLIDATOR] = "NA" then " " else [DECONSOLIDATOR]),
    #"Added CarrierA_DDC_SCAC" = Table.AddColumn(#"Added DeConsol_cd", "CarrierA_DDC_SCAC", each if [DECON TO PLT CARRIER SCAC] = " " then " " else if [DECON TO PLT CARRIER SCAC] = "FCL" then " " else if [DECONSOLIDATOR] = 0 then " " else if [DECON TO PLT CARRIER SCAC] = "NA" then " " else [DECON TO PLT CARRIER SCAC]),
    #"Added CarrierA_DDC_GSDB6" = Table.AddColumn(#"Added CarrierA_DDC_SCAC", "CarrierA_DDC_GSDB6", each if [POST DECONSOL STOP] = " " then " " else if [POST DECONSOL STOP] = "FCL" then " " else if [POST DECONSOL STOP] = 0 then " " else if [POST DECONSOL STOP] = "NA" then " " else [POST DECONSOL STOP]),
    #"Added Dock_Code" = Table.AddColumn(#"Added CarrierA_DDC_GSDB6", "Dock_Code", each if [DOCK CODE] = "NA" then " " else if [DOCK CODE] = 0 then " " else [DOCK CODE]),
    #"Added Alternate_ShipTo" = Table.AddColumn(#"Added Dock_Code", "Alternate_ShipTo", each if [ALTCONSIGNEE] = " " then " " else if [ALTCONSIGNEE] = "NA"  then " " else if [ALTCONSIGNEE] = 0 then " " else [ALTCONSIGNEE]),
    #"Added Payment_Terms" = Table.AddColumn(#"Added Alternate_ShipTo", "Payment_Terms", each [PAYMENT TERMS]),
    //slicing the values in the provided columns
    #"Extracted First Characters Payment_Terms" = Table.TransformColumns(#"Added Payment_Terms", {{"Payment_Terms", each Text.Start(Text.From(_, "en-US"), 3), type text}}),
    #"Added LoadYd_Cutoff_Time" = Table.AddColumn(#"Extracted First Characters Payment_Terms", "LoadYd_Cutoff_Time", each if [#"LOAD YARD CUT-OFF"] = 0 then " " else [#"LOAD YARD CUT-OFF"]),
    #"Inserted Last Characters LoadYd_Cutoff_Time" = Table.AddColumn(#"Added LoadYd_Cutoff_Time", "Last Characters", each Text.End([LoadYd_Cutoff_Time], 4), type text),
    #"Inserted Last Characters Origin_Port" = Table.AddColumn(#"Inserted Last Characters LoadYd_Cutoff_Time", "Last Characters.1", each Text.End([Origin_Port], 3), type text),
    #"Added Destn_Port" = Table.AddColumn(#"Inserted Last Characters Origin_Port", "Destn_Port", each [Origin_Port]),
    #"Inserted Last Characters" = Table.AddColumn(#"Added Destn_Port", "Last Characters.2", each Text.End([Destn_Port], 3), type text),
    //added in the event day from the days table (replaced the vlookups in the sheet)
    #"First Characters LOAD YARD CUT-OFF" = Table.TransformColumns(#"Inserted Last Characters", {{"LOAD YARD CUT-OFF", each Text.Start(_, 3), type text}}),
    #"Merged LoadYd_Cutoff_Day" = Table.NestedJoin(#"First Characters LOAD YARD CUT-OFF", {"LOAD YARD CUT-OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Renamed LoadYd_Cutoff_Day" = Table.RenameColumns(#"Merged LoadYd_Cutoff_Day",{{"days", "LoadYd_Cutoff_Day"}}),
    #"Expanded LoadYd_Cutoff_Day" = Table.ExpandTableColumn(#"Renamed LoadYd_Cutoff_Day", "LoadYd_Cutoff_Day", {"Letter"}, {"LoadYd_Cutoff_Day.Letter"}),
    #"Extracted First Characters" = Table.TransformColumns(#"Expanded LoadYd_Cutoff_Day", {{"LoadYd_Cutoff_Day.Letter", each Text.Start(_, 2), type text}}),
    #"Renamed Columns LoadYd_Cutoff_Day" = Table.RenameColumns(#"Extracted First Characters",{{"LoadYd_Cutoff_Day.Letter", "LoadYd_Cutoff_Day"}}),
    #"Added PORT CUT OFF COPY" = Table.AddColumn(#"Renamed Columns LoadYd_Cutoff_Day", "PORT CUT OFF COPY", each [PORT CUT OFF]),
    #"First Characters PORT CUT OFF" = Table.TransformColumns(#"Added PORT CUT OFF COPY", {{"PORT CUT OFF", each Text.Start(_, 3), type text}}),
    #"Merged Port_Cutoff_Day" = Table.NestedJoin(#"First Characters PORT CUT OFF", {"PORT CUT OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded days" = Table.ExpandTableColumn(#"Merged Port_Cutoff_Day", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed Port_Cutoff_Time" = Table.RenameColumns(#"Expanded days",{{"days.numeric", "Port_Cutoff_Time"}}),
    #"First Characters ESTIMATED SA DAY" = Table.TransformColumns(#"Renamed Port_Cutoff_Time", {{"ESTIMATED SA DAY", each Text.Start(_, 3), type text}}),
    #"Merged Est_Sail_Day" = Table.NestedJoin(#"First Characters ESTIMATED SA DAY", {"ESTIMATED SA DAY"}, days, {"Day"}, "days.1", JoinKind.LeftOuter),
    #"Expanded Est_Sail_Day" = Table.ExpandTableColumn(#"Merged Est_Sail_Day", "days.1", {"numeric"}, {"days.1.numeric"}),
    #"Renamed Est_Sail_Day" = Table.RenameColumns(#"Expanded Est_Sail_Day",{{"days.1.numeric", "Est_Sail_Day"}}),
    #"First Characters ESTIMATED AR DAY" = Table.TransformColumns(#"Renamed Est_Sail_Day", {{"ESTIMATED AR DAY", each Text.Start(_, 3), type text}}),
    #"Merged Est_Arr_Day_Dest_Port" = Table.NestedJoin(#"First Characters ESTIMATED AR DAY", {"ESTIMATED AR DAY"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded Est_Arr_Day_Dest_Port" = Table.ExpandTableColumn(#"Merged Est_Arr_Day_Dest_Port", "days", {"numeric"}, {"days.numeric"}),
    #"First Characters Est_Arr_Day_Dest_Port" = Table.TransformColumns(#"Expanded Est_Arr_Day_Dest_Port", {{"days.numeric", each Text.Start(_, 2), type text}}),
    #"Renamed Est_Arr_Day_Dest_Port" = Table.RenameColumns(#"First Characters Est_Arr_Day_Dest_Port",{{"days.numeric", "Est_Arr_Day_Dest_Port"}}),
    #"Merged port_cutoff" = Table.NestedJoin(#"Renamed Est_Arr_Day_Dest_Port", {"PORT CUT OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded port_cutoff" = Table.ExpandTableColumn(#"Merged port_cutoff", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed port_cutoff" = Table.RenameColumns(#"Expanded port_cutoff",{{"days.numeric", "port_cutoff"}}),
    #"First Characters port_cutoff" = Table.TransformColumns(#"Renamed port_cutoff", {{"port_cutoff", each Text.Start(_, 2), type text}}),
    //recreated frequency day logic as used in the routing guide
    #"Added Frequency_Day" = Table.AddColumn(#"First Characters port_cutoff", "Frequency_Day ", each if ([#"ship_freq,"] = 31) or ([#"ship_freq,"] = 32) or ([#"ship_freq,"] = 41) then [#"ship_freq,"] 
    else if [#"ship_freq,"] = 21 then 1 else if [#"ship_freq,"] = 19 then 6 
    else if [#"ship_freq,"] = 22 then 2 else if [#"ship_freq,"] = 23 then 3 
    else if [#"ship_freq,"] = 24 then 4 else if [#"ship_freq,"] = 25 then 5 else 8),
    #"Added Export_Total" = Table.AddColumn(#"Added Frequency_Day", "Export_Total", each 
    if [Origin_Load_Type] = "LCL" then ([#"DAYS SFS TO ACC (LCL)"] + [#"DAYS ACC TO DCC (LCL)"] + [#"DAYS DCC TO AOP (LCL)"] + [#"aop_otb,"]) else ([#"DAYS SFS TO AOP (FCL)"] + [#"aop_otb,"])),
    #"Added Import_Total" = Table.AddColumn(#"Added Export_Total", "Import_Total", each if [Destn_Load_Type] = "LCL" then ([#"adp_tld,"] + [#"DAYS TLD TO ADC (LCL)"] + [#"DAYS ADC TO DDC (LCL)"] + [#"DAYS DDC TO DPL (LCL)"]) else ([#"adp_tld,"] + [#"DAYS TLD TO DPL (FCL)"])),
    #"Added CarrierA_SFS_SCAC" = Table.AddColumn(#"Added Import_Total", "CarrierA_SFS_SCAC", each if [SUP TO CONSOL CARRIER SCAC] = "NA" then "" else if [SUP TO CONSOL CARRIER SCAC] = "DHL" then "" else if [SUP TO CONSOL CARRIER SCAC] = "FCL" then "" else [SUP TO CONSOL CARRIER SCAC]),
    #"Added Consol_Cutoff_Time" = Table.AddColumn(#"Added CarrierA_SFS_SCAC", "Consol_Cutoff_Time", each if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = 0 then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = " " 
    then [#"CONSOLIDATOR (POOL) CUT-OFF time"] = "NA" else if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = "None" then " " else [#"CONSOLIDATOR (POOL) CUT-OFF time"]),
    #"Added CarrierA_TLD_GSDB6" = Table.AddColumn(#"Added Consol_Cutoff_Time", "CarrierA_TLD_GSDB6", each [SSL_GSDB6]),
    #"Added Transport_Total" = Table.AddColumn(#"Added CarrierA_TLD_GSDB6", "Transport_Total", each [#"otb_adp,"]),
    //recreated the if statement including the V-Lookup to create the Consol_Cutoff_Day column
    #"Merged Consol_Cutoff_Day (merge)" = Table.NestedJoin(#"Added Transport_Total", {"CONSOLIDATOR (POOL) CUT-OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded Consol_Cutoff_Day (merge)" = Table.ExpandTableColumn(#"Merged Consol_Cutoff_Day (merge)", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed Merged Consol_Cutoff_Day (merge)" = Table.RenameColumns(#"Expanded Consol_Cutoff_Day (merge)",{{"days.numeric", "Merged Consol_Cutoff_Day (merge)"}}),
    #"Added Consol_Cutoff_Day" = Table.AddColumn(#"Renamed Merged Consol_Cutoff_Day (merge)", "Consol_Cutoff_Day", each if [#"CONSOLIDATOR (POOL) CUT-OFF"] = 0 then " " else 
    if [#"CONSOLIDATOR (POOL) CUT-OFF"] = " " then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF"] = "NA" then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF"] = "None" then " " else [days.numeric]),
    #"Removed Merged Consol_Cutoff_Day (merge)" = Table.RemoveColumns(#"Added Consol_Cutoff_Day",{"Merged Consol_Cutoff_Day (merge)"}),
    //adding in the blank columns to make sure we recreate the exact same output file to upload in the Ford system
    #"Added Broker_cd" = Table.AddColumn(#"Removed Merged Consol_Cutoff_Day (merge)", "Broker_cd", each ""),
    #"Added Broker_SCAC" = Table.AddColumn(#"Added Broker_cd", "Broker_SCAC", each ""),
    #"Added Other_Exception" = Table.AddColumn(#"Added Broker_SCAC", "Other_Exception", each ""),
    #"Added Pre-consol_stop_GSDB" = Table.AddColumn(#"Added Other_Exception", "Pre-consol_stop_GSDB", each ""),
    #"Added CarrierA_SFS_GSDB6" = Table.AddColumn(#"Added Pre-consol_stop_GSDB", "CarrierA_SFS_GSDB6", each ""),
    #"Added Consol_SCAC" = Table.AddColumn(#"Added CarrierA_SFS_GSDB6", "Consol_SCAC", each ""),
    #"Added Container_Type_Other" = Table.AddColumn(#"Added Consol_SCAC", "Container_Type_Other", each ""),
    #"Added LoadYard_GSDB6_cd" = Table.AddColumn(#"Added Container_Type_Other", "LoadYard_GSDB6_cd", each ""),
    #"Added CarrierA_DDC_Mode" = Table.AddColumn(#"Added LoadYard_GSDB6_cd", "CarrierA_DDC_Mode", each "T"),
    //port cutoff logic I added two columns to allow me to recreate the focus on the last four digits in the original excel file
    #"Added PORT CUT OFF COPY (last 4 values)" = Table.AddColumn(#"Added CarrierA_DDC_Mode", "PORT CUT OFF COPY (last 4 values)", each [PORT CUT OFF COPY]),
    #"Inserted Last Characters1" = Table.AddColumn(#"Added PORT CUT OFF COPY (last 4 values)", "Last Characters.3", each Text.End([#"PORT CUT OFF COPY (last 4 values)"], 4), type text),
    #"Added PORT CUT OFF 2" = Table.AddColumn(#"Inserted Last Characters1", "PORT CUT OFF 2", each if [Last Characters.3] = "0000" then " " else [PORT CUT OFF COPY]),
    #"Removed Columns" = Table.RemoveColumns(#"Added PORT CUT OFF 2",{"PORT CUT OFF COPY (last 4 values)", "Last Characters.3"})
in
    #"Removed Columns"




    working on

    let
    Source = master_old_data,
    #"Changed Type" = Table.TransformColumnTypes(Source,{{"2346", type text}}),
    #"Renamed Columns" = Table.RenameColumns(#"Changed Type",{{"MODE", "mmode"}, {"SUPPLIER GSDB", "supplier_cd"}, {"PLANT GSDB", "plant_cd"}, {"COMMODITY", "commodity"}, 
    {"SHIP FREQUENCY", "ship_freq,"}, {"FORD ROUTE ID", "route_id,"}, {"TOTAL DAYS ROUTE GUIDE", "total_transit,"}, {"DAYS AOP TO OTB", "aop_otb,"}, {"DAYS OTB TO ADP", "otb_adp,"}, 
    {"DAYS ADP TO TLD", "adp_tld,"}, {"START EFFECTIVE YEAR", "trash1"}, {"START EFFECTIVE MONTH", "trash2"}, {"START EFFECTIVE DAY", "trash3"}, {"END EFFECTIVE YEAR", "trash4"}, 
    {"END EFFECTIVE MONTH", "trash5"}, {"END EFFECTIVE DAY", "trash6"}, {"TDI MODE", "Mode"}, {"DIRECTION", "Direction"}, {"ADDITIONAL CMMS EXCEPTION TRANSIT", "Exception_Total"}, 
    {"EXCEPTION TRANSIT REASON", "Exception_Reason"}, {"MODE TO PORT", "CarrierA_SFS_Mode"}, {"CONSOLIDATOR GSDB", "Consol_cd"}, {"SSL GSDB", "SSL_GSDB6"},  
    {"SSL CONTRACT NUMBER", "Contract_Number"}, {"% OF BUSINESS WITH SSL", "SSL_Percent"}, {"SSL PICK UP LOC", "SSL_Pickup_cd"}, {"SSL PICK UP LOAD TYPE", "SSL_Pickup_Type"}, {"MODE FROM PORT", "CarrierA_TLD_Mode"}, 
    {"SSL DROP OFF UNLOAD TYPE", "Delivery_Type"}, {"SSL DROP OFF LOC", "SSL_DropOff_GSDB"}, {"PRIORITY", "Priority"}, {"TOTAL DAYS CMMS", "cmms_transit,"}, {"NEW PORT CODE", "Origin_Port"}}),
    //rounded off the float int he column
    #"Rounded Off cmms_transit," = Table.TransformColumns(#"Renamed Columns",{{"cmms_transit,", each Number.Round(_, 0), Int64.Type}}),
    // merged in the code sheet
    #"Merged Queries" = Table.NestedJoin(#"Rounded Off cmms_transit,", {"mmode"}, Codes, {"Code"}, "Codes", JoinKind.LeftOuter),
    #"Expanded Codes" = Table.ExpandTableColumn(#"Merged Queries", "Codes", {"Export", "Import"}, {"Codes.Export", "Codes.Import"}),
    #"Renamed load_type columns" = Table.RenameColumns(#"Expanded Codes",{{"Codes.Export", "Origin_Load_Type"}, {"Codes.Import", "Destn_Load_Type"}}),
    //rewrote the excel formulas used in the routingguide
    #"Added sfs_acc," = Table.AddColumn(#"Renamed load_type columns", "sfs_acc,", each if [Origin_Load_Type] = "LCL" then [#"DAYS SFS TO ACC (LCL)"] else 999),
    #"Added acc_dcc," = Table.AddColumn(#"Added sfs_acc,", "acc_dcc,", each if [Origin_Load_Type] = "LCL" then [#"DAYS ACC TO DCC (LCL)"] else 999),
    #"Added tld_adc," = Table.AddColumn(#"Added acc_dcc,", "tld_adc,", each if [Destn_Load_Type] = "LCL" then [#"DAYS TLD TO ADC (LCL)"] else "999"),
    #"Added adc_ddc," = Table.AddColumn(#"Added tld_adc,", "adc_ddc,", each if [Destn_Load_Type] = "LCL" then [#"DAYS ADC TO DDC (LCL)"] else "999"),
    #"Added dcc_aop," = Table.AddColumn(#"Added adc_ddc,", "dcc_aop,", each if [Origin_Load_Type] = "LCL" then [#"DAYS DCC TO AOP (LCL)"] else [#"DAYS SFS TO AOP (FCL)"]),
    #"Added ddc_dpl," = Table.AddColumn(#"Added dcc_aop,", "ddc_dpl,", each if [Destn_Load_Type] = "LCL" then [#"DAYS DDC TO DPL (LCL)"] else [#"DAYS TLD TO DPL (FCL)"]),
    //used nested if statements to recreate an excel or operator
    #"Added DeConsol_cd" = Table.AddColumn(#"Added ddc_dpl,", "DeConsol_cd", each if [DECONSOLIDATOR] = " " then " " else if [DECONSOLIDATOR] = "FCL" then " " else if [DECONSOLIDATOR] = 0 then " " else if [DECONSOLIDATOR] = "NA" then " " else [DECONSOLIDATOR]),
    #"Added CarrierA_DDC_SCAC" = Table.AddColumn(#"Added DeConsol_cd", "CarrierA_DDC_SCAC", each if [DECON TO PLT CARRIER SCAC] = " " then " " else if [DECON TO PLT CARRIER SCAC] = "FCL" then " " else if [DECONSOLIDATOR] = 0 then " " else if [DECON TO PLT CARRIER SCAC] = "NA" then " " else [DECON TO PLT CARRIER SCAC]),
    #"Added CarrierA_DDC_GSDB6" = Table.AddColumn(#"Added CarrierA_DDC_SCAC", "CarrierA_DDC_GSDB6", each if [POST DECONSOL STOP] = " " then " " else if [POST DECONSOL STOP] = "FCL" then " " else if [POST DECONSOL STOP] = 0 then " " else if [POST DECONSOL STOP] = "NA" then " " else [POST DECONSOL STOP]),
    #"Added Dock_Code" = Table.AddColumn(#"Added CarrierA_DDC_GSDB6", "Dock_Code", each if [DOCK CODE] = "NA" then " " else if [DOCK CODE] = 0 then " " else [DOCK CODE]),
    #"Added Alternate_ShipTo" = Table.AddColumn(#"Added Dock_Code", "Alternate_ShipTo", each if [ALTCONSIGNEE] = " " then " " else if [ALTCONSIGNEE] = "NA"  then " " else if [ALTCONSIGNEE] = 0 then " " else [ALTCONSIGNEE]),
    #"Added Payment_Terms" = Table.AddColumn(#"Added Alternate_ShipTo", "Payment_Terms", each [PAYMENT TERMS]),
    //slicing the values in the provided columns
    #"Extracted First Characters Payment_Terms" = Table.TransformColumns(#"Added Payment_Terms", {{"Payment_Terms", each Text.Start(Text.From(_, "en-US"), 3), type text}}),
    #"Added LoadYd_Cutoff_Time" = Table.AddColumn(#"Extracted First Characters Payment_Terms", "LoadYd_Cutoff_Time", each if [#"LOAD YARD CUT-OFF"] = 0 then " " else [#"LOAD YARD CUT-OFF"]),
    #"Inserted Last Characters LoadYd_Cutoff_Time" = Table.AddColumn(#"Added LoadYd_Cutoff_Time", "Last Characters", each Text.End([LoadYd_Cutoff_Time], 4), type text),
    #"Inserted Last Characters Origin_Port" = Table.AddColumn(#"Inserted Last Characters LoadYd_Cutoff_Time", "Last Characters.1", each Text.End([Origin_Port], 3), type text),
    #"Added Destn_Port" = Table.AddColumn(#"Inserted Last Characters Origin_Port", "Destn_Port", each [Origin_Port]),
    #"Inserted Last Characters" = Table.AddColumn(#"Added Destn_Port", "Last Characters.2", each Text.End([Destn_Port], 3), type text),
    //added in the event day from the days table (replaced the vlookups in the sheet)
    #"First Characters LOAD YARD CUT-OFF" = Table.TransformColumns(#"Inserted Last Characters", {{"LOAD YARD CUT-OFF", each Text.Start(_, 3), type text}}),
    #"Merged LoadYd_Cutoff_Day" = Table.NestedJoin(#"First Characters LOAD YARD CUT-OFF", {"LOAD YARD CUT-OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Renamed LoadYd_Cutoff_Day" = Table.RenameColumns(#"Merged LoadYd_Cutoff_Day",{{"days", "LoadYd_Cutoff_Day"}}),
    #"Expanded LoadYd_Cutoff_Day" = Table.ExpandTableColumn(#"Renamed LoadYd_Cutoff_Day", "LoadYd_Cutoff_Day", {"Letter"}, {"LoadYd_Cutoff_Day.Letter"}),
    #"Extracted First Characters" = Table.TransformColumns(#"Expanded LoadYd_Cutoff_Day", {{"LoadYd_Cutoff_Day.Letter", each Text.Start(_, 2), type text}}),
    #"Renamed Columns LoadYd_Cutoff_Day" = Table.RenameColumns(#"Extracted First Characters",{{"LoadYd_Cutoff_Day.Letter", "LoadYd_Cutoff_Day"}}),
    #"Added PORT CUT OFF COPY" = Table.AddColumn(#"Renamed Columns LoadYd_Cutoff_Day", "PORT CUT OFF COPY", each [PORT CUT OFF]),
    #"First Characters PORT CUT OFF" = Table.TransformColumns(#"Added PORT CUT OFF COPY", {{"PORT CUT OFF", each Text.Start(_, 3), type text}}),
    #"Merged Port_Cutoff_Day" = Table.NestedJoin(#"First Characters PORT CUT OFF", {"PORT CUT OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded days" = Table.ExpandTableColumn(#"Merged Port_Cutoff_Day", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed Port_Cutoff_Time" = Table.RenameColumns(#"Expanded days",{{"days.numeric", "Port_Cutoff_Time"}}),
    #"First Characters ESTIMATED SA DAY" = Table.TransformColumns(#"Renamed Port_Cutoff_Time", {{"ESTIMATED SA DAY", each Text.Start(_, 3), type text}}),
    #"Merged Est_Sail_Day" = Table.NestedJoin(#"First Characters ESTIMATED SA DAY", {"ESTIMATED SA DAY"}, days, {"Day"}, "days.1", JoinKind.LeftOuter),
    #"Expanded Est_Sail_Day" = Table.ExpandTableColumn(#"Merged Est_Sail_Day", "days.1", {"numeric"}, {"days.1.numeric"}),
    #"Renamed Est_Sail_Day" = Table.RenameColumns(#"Expanded Est_Sail_Day",{{"days.1.numeric", "Est_Sail_Day"}}),
    #"First Characters ESTIMATED AR DAY" = Table.TransformColumns(#"Renamed Est_Sail_Day", {{"ESTIMATED AR DAY", each Text.Start(_, 3), type text}}),
    #"Merged Est_Arr_Day_Dest_Port" = Table.NestedJoin(#"First Characters ESTIMATED AR DAY", {"ESTIMATED AR DAY"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded Est_Arr_Day_Dest_Port" = Table.ExpandTableColumn(#"Merged Est_Arr_Day_Dest_Port", "days", {"numeric"}, {"days.numeric"}),
    #"First Characters Est_Arr_Day_Dest_Port" = Table.TransformColumns(#"Expanded Est_Arr_Day_Dest_Port", {{"days.numeric", each Text.Start(_, 2), type text}}),
    #"Renamed Est_Arr_Day_Dest_Port" = Table.RenameColumns(#"First Characters Est_Arr_Day_Dest_Port",{{"days.numeric", "Est_Arr_Day_Dest_Port"}}),
    #"Merged port_cutoff" = Table.NestedJoin(#"Renamed Est_Arr_Day_Dest_Port", {"PORT CUT OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded port_cutoff" = Table.ExpandTableColumn(#"Merged port_cutoff", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed port_cutoff" = Table.RenameColumns(#"Expanded port_cutoff",{{"days.numeric", "port_cutoff"}}),
    #"First Characters port_cutoff" = Table.TransformColumns(#"Renamed port_cutoff", {{"port_cutoff", each Text.Start(_, 2), type text}}),
    //recreated frequency day logic as used in the routing guide
    #"Added Frequency_Day" = Table.AddColumn(#"First Characters port_cutoff", "Frequency_Day ", each if ([#"ship_freq,"] = 31) or ([#"ship_freq,"] = 32) or ([#"ship_freq,"] = 41) then [#"ship_freq,"] 
    else if [#"ship_freq,"] = 21 then 1 else if [#"ship_freq,"] = 19 then 6 
    else if [#"ship_freq,"] = 22 then 2 else if [#"ship_freq,"] = 23 then 3 
    else if [#"ship_freq,"] = 24 then 4 else if [#"ship_freq,"] = 25 then 5 else 8),
    #"Added Export_Total" = Table.AddColumn(#"Added Frequency_Day", "Export_Total", each 
    if [Origin_Load_Type] = "LCL" then ([#"DAYS SFS TO ACC (LCL)"] + [#"DAYS ACC TO DCC (LCL)"] + [#"DAYS DCC TO AOP (LCL)"] + [#"aop_otb,"]) else ([#"DAYS SFS TO AOP (FCL)"] + [#"aop_otb,"])),
    #"Added Import_Total" = Table.AddColumn(#"Added Export_Total", "Import_Total", each if [Destn_Load_Type] = "LCL" then ([#"adp_tld,"] + [#"DAYS TLD TO ADC (LCL)"] + [#"DAYS ADC TO DDC (LCL)"] + [#"DAYS DDC TO DPL (LCL)"]) else ([#"adp_tld,"] + [#"DAYS TLD TO DPL (FCL)"])),
    #"Added CarrierA_SFS_SCAC" = Table.AddColumn(#"Added Import_Total", "CarrierA_SFS_SCAC", each if [SUP TO CONSOL CARRIER SCAC] = "NA" then "" else if [SUP TO CONSOL CARRIER SCAC] = "DHL" then "" else if [SUP TO CONSOL CARRIER SCAC] = "FCL" then "" else [SUP TO CONSOL CARRIER SCAC]),
    #"Added Consol_Cutoff_Time" = Table.AddColumn(#"Added CarrierA_SFS_SCAC", "Consol_Cutoff_Time", each if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = 0 then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = " " 
    then [#"CONSOLIDATOR (POOL) CUT-OFF time"] = "NA" else if [#"CONSOLIDATOR (POOL) CUT-OFF time"] = "None" then " " else [#"CONSOLIDATOR (POOL) CUT-OFF time"]),
    #"Added CarrierA_TLD_GSDB6" = Table.AddColumn(#"Added Consol_Cutoff_Time", "CarrierA_TLD_GSDB6", each [SSL_GSDB6]),
    #"Added Transport_Total" = Table.AddColumn(#"Added CarrierA_TLD_GSDB6", "Transport_Total", each [#"otb_adp,"]),
    //recreated the if statement including the V-Lookup to create the Consol_Cutoff_Day column
    #"Merged Consol_Cutoff_Day (merge)" = Table.NestedJoin(#"Added Transport_Total", {"CONSOLIDATOR (POOL) CUT-OFF"}, days, {"Day"}, "days", JoinKind.LeftOuter),
    #"Expanded Consol_Cutoff_Day (merge)" = Table.ExpandTableColumn(#"Merged Consol_Cutoff_Day (merge)", "days", {"numeric"}, {"days.numeric"}),
    #"Renamed Merged Consol_Cutoff_Day (merge)" = Table.RenameColumns(#"Expanded Consol_Cutoff_Day (merge)",{{"days.numeric", "Merged Consol_Cutoff_Day (merge)"}}),
    #"Added Consol_Cutoff_Day" = Table.AddColumn(#"Renamed Merged Consol_Cutoff_Day (merge)", "Consol_Cutoff_Day", each if [#"CONSOLIDATOR (POOL) CUT-OFF"] = 0 then " " else 
    if [#"CONSOLIDATOR (POOL) CUT-OFF"] = " " then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF"] = "NA" then " " else if [#"CONSOLIDATOR (POOL) CUT-OFF"] = "None" then " " else [days.numeric]),
    #"Removed Merged Consol_Cutoff_Day (merge)" = Table.RemoveColumns(#"Added Consol_Cutoff_Day",{"Merged Consol_Cutoff_Day (merge)"}),
    //adding in the blank columns to make sure we recreate the exact same output file to upload in the Ford system
    #"Added Broker_cd" = Table.AddColumn(#"Removed Merged Consol_Cutoff_Day (merge)", "Broker_cd", each ""),
    #"Added Broker_SCAC" = Table.AddColumn(#"Added Broker_cd", "Broker_SCAC", each ""),
    #"Added Other_Exception" = Table.AddColumn(#"Added Broker_SCAC", "Other_Exception", each ""),
    #"Added Pre-consol_stop_GSDB" = Table.AddColumn(#"Added Other_Exception", "Pre-consol_stop_GSDB", each ""),
    #"Added CarrierA_SFS_GSDB6" = Table.AddColumn(#"Added Pre-consol_stop_GSDB", "CarrierA_SFS_GSDB6", each ""),
    #"Added Consol_SCAC" = Table.AddColumn(#"Added CarrierA_SFS_GSDB6", "Consol_SCAC", each ""),
    #"Added Container_Type_Other" = Table.AddColumn(#"Added Consol_SCAC", "Container_Type_Other", each ""),
    #"Added LoadYard_GSDB6_cd" = Table.AddColumn(#"Added Container_Type_Other", "LoadYard_GSDB6_cd", each ""),
    #"Added CarrierA_DDC_Mode" = Table.AddColumn(#"Added LoadYard_GSDB6_cd", "CarrierA_DDC_Mode", each "T"),
    //port cutoff logic I added two columns to allow me to recreate the focus on the last four digits in the original excel file
    #"Added PORT CUT OFF COPY (last 4 values)" = Table.AddColumn(#"Added CarrierA_DDC_Mode", "PORT CUT OFF COPY (last 4 values)", each [PORT CUT OFF COPY]),
    #"Inserted Last Characters1" = Table.AddColumn(#"Added PORT CUT OFF COPY (last 4 values)", "Last Characters.3", each Text.End([#"PORT CUT OFF COPY (last 4 values)"], 4), type text),
    #"Added PORT CUT OFF 2" = Table.AddColumn(#"Inserted Last Characters1", "PORT CUT OFF 2", each if [Last Characters.3] = "0000" then " " else [PORT CUT OFF COPY]),
    #"Removed Columns" = Table.RemoveColumns(#"Added PORT CUT OFF 2",{"PORT CUT OFF COPY (last 4 values)", "Last Characters.3", "2346"}),
    //ordering columns to match milestone feed
    #"Reordered Columns" = Table.ReorderColumns(#"Removed Columns",{"mmode", "supplier_cd", "plant_cd", "commodity", "ship_freq,", "route_id,", "cmms_transit,", "port_cutoff",
    "total_transit,","sfs_acc,", "acc_dcc,", "dcc_aop,", "aop_otb,", "otb_adp,", "adp_tld,", "tld_adc,", "adc_ddc,", "ddc_dpl,",  "trash1", "trash2", "trash3", "trash4", "trash5", "trash6", 
     //week_nbr
     "Mode", "Origin_Load_Type", "Destn_Load_Type", "Direction", "Payment_Terms", "Broker_cd", "Broker_SCAC", "Frequency_Day ", "Exception_Total", "Exception_Reason", "Other_Exception", 
    "Export_Total", "Import_Total", "Transport_Total","Pre-consol_stop_GSDB",  "CarrierA_SFS_SCAC", "CarrierA_SFS_GSDB6", "CarrierA_SFS_Mode",
    "Consol_cd","Consol_SCAC","Consol_Cutoff_Day", "Consol_Cutoff_Time", "SSL_GSDB6",
    //"SSL_SCAC", 
    "Contract_Number",
    //SSL_Percent
    "SSL_Pickup_cd","SSL_Pickup_Type", "Container_Type_20", "Container_Type_40    ", "Container_Type_40HC    ", "Container_Type_45    ","Container_Type_Other","LoadYard_GSDB6_cd",
    "LoadYd_Cutoff_Day",  "LoadYd_Cutoff_Time",
    //Port_Cutoff_Day
    "Port_Cutoff_Time", "Origin_Port", "Est_Sail_Day", "Destn_Port", "Est_Arr_Day_Dest_Port",
    //"CarrierA_TLD_SCAC",
    "CarrierA_TLD_GSDB6","CarrierA_TLD_Mode", "Delivery_Type",  "SSL_DropOff_GSDB","DeConsol_cd", "CarrierA_DDC_SCAC","CarrierA_DDC_GSDB6","CarrierA_DDC_Mode","Dock_Code","Alternate_ShipTo", "Priority", "Routing_Descriptor ",


    
    "SUPPLIER CMF", "PLANT CMF", 
    "1 DIGIT DAY",  "REGION", "Region Code", "Country Code", "FLOAT (CMMS - ROUTE GUIDE)",  "DAYS SFS TO AOP (FCL)", "DAYS SFS TO ACC (LCL)", "DAYS ACC TO DCC (LCL)", 
    "DAYS DCC TO AOP (LCL)", "SUPPLIER", "SUPPLIER BUSINESS NAME", "STREET", "CITY", "COUNTRY", "POSTAL CODE", "CONTACT NAME", "PHONE NUMBER", "EMAIL ADDRESS 1", "EMAIL ADDRESS 2", "EMAIL ADDRESS 3", 
    
    //"SSL PICK UP LOC", 
    "SSL PICK UP LOC NAME", "SSL PICK UP LOC ADDRESS", "SSL PICK UP LOC CITY", "SSL PICK UP LOC ST, PR, TE", "SSL PICK UP COUNTRY", "SSL PICKUP POSTAL CODE",  
     "CONSOLIDATOR",  "Carrier#(lf)(Direct / Pre-#(lf)ODC/CC#(lf)collection)", 

    "CARRIER CONTACT SURNAME", "CARRIER CONTACT NAME", "CARRIER CONTACT LOCATION", "CARRIER PHONE", "CARRIER FAX", "CARRIER E-MAIL",  "CARRIER MOBILE", "1st#(lf)ODC/CC", 
    "LTL#(lf)Carrier#(lf)ex 1st ODC/CC", "2nd#(lf)ODC/CC", "LTL#(lf)Carrier#(lf)ex2nd ODC/CC", "SUP TO CONSOL CARRIER SCAC", "PRE CONSOL STOP", "CONSOLIDATOR (POOL) CUT-OFF", 

    "CONSOLIDATOR (POOL) CUT-OFF time",  "LOAD YARD LOCATION", "LOAD YARD CUT-OFF", "PORT CUT OFF", "ORIGIN PORT",    
    "ESTIMATED SA DAY", "ESTIMATED AR DAY", "SSL NAME",    "INTERMITTENT PORT", "DESTINATION PORT", "NEW PORT CODE_1",

    "YARD LOCATION UNLOAD",  "SSL DROP OFF LOCATION NAME", "SSL DROP OFF LOC ADDRESS", "SSL DROP OFF LOC CITY", "SSL DROP OF LOC ST, PR, TR", 
    "SSL DROP OFF LOC CON CODE", "SSL DROP OFF LOC POSTAL CODE",  "DECONSOLIDATOR", "DECON TO PLT CARRIER SCAC", "POST DECONSOL STOP", "DAYS TLD TO ADC (LCL)", "DAYS ADC TO DDC (LCL)", 

    "DAYS DDC TO DPL (LCL)", "DAYS TLD TO DPL (FCL)", "ALTCONSIGNEE", "PLANT", "PLANT BUSINESS NAME", "PLANT STREET NAME", "CITY_2", "STATE, PROV, TERR", "COUNTRY_3", "POSTAL CODE_4", "DOCK CODE", 
     "ADDITIONAL RG EXCEPTION TRANSIT",  "CMMS TT W/O EXCEPTION TRANSIT", "RG TT W/O EXCEPTION TRANSIT ",

    "CUSTOMS BROKER", "PAYMENT TERMS",  "ACTIVE/INACTIVE",   "UPDATE FLAG FOR SHIPPING INSTRUCTIONS", "EMAIL ADDRESS 4", "STATE, PROV, TERR_5", "NAME OF CARRIER'S SERVICE",
     "MARKUS CMMS TIME", "TOTAL DAYS CMMS_6", "MARKUS SF", "SHIP FREQUENCY_7", "MARKUS DIGIT DAY", "1 DIGIT DAY_8", "DISCREPANCY", "RESPONSIBLE PARTY", "WEEK NUMBER", "MARKUS COMMENTS",  

               
     "Last Characters", "Last Characters.1", "Last Characters.2",  "PORT CUT OFF COPY",    
           
           "PORT CUT OFF 2"}),
    #"Added week_nbr" = Table.AddColumn(#"Reordered Columns", "week_nbr", each "2243")
in
    #"Added week_nbr"