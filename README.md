# Power Query
Notes from projects I worked on where creating a excel file with Power Query was the best solution.

While working at DSV I found that ofthen I worked with various teams without team members with a coding background. We did several projects to create excel files for the analysis of data and to create feedback reports on the quality of manually generated data in CargoWise1. Writing these scripts in M-Code in excel files allow me to share them across teams without the need to have them install any software. I could not achieve the same with Python scripts as the EXE wrappers I used to wrap my Python scripts are blocked by the DSV firewall. I found I could get the same result with minimal resistance by writing the logic in a excel file that I can share with other teams allowing them to do work with the scripts.

An added advantage is that with Sharepoint and Power automate the logic in the scripts can be automated on a basic lavel allowing team memmbers to receive run reports with the analysis/data they need.

I am using the repository to store the M-Code allowing me to easily read back on code I have written in the past as I am a big copy paster. 

The routing guide was a project to automate a cumbersome pricing/routing excel file is used between teams

The error sheets are sheet where I identified error in the input data and processed it so it would be presented in the folling format:
responcible location/operator/shipment/error. I found the original way of presenting the high level amound of errors made resolving the errors for the operators very cumbersome and they dragged their feed because of it. In this format it is easier to identify the errors and the shipments they are on and have the responcible operators correct them. We also tied these to KPI's for the operators. 
