import Algorithmia
import urllib.request
import pandas as pd
import os
import time
import json
import time
import os
import docx
from docx import Document
from docx.shared import Pt
from reportlab.lib.units import inch
from reportlab.pdfgen import canvas
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Table, TableStyle, PageBreak
from reportlab.lib import colors
from reportlab.lib.pagesizes import letter
from itertools import groupby
import json
import ssl


#######API INFORMATION##########
# BVD_user = 'E&YUSws'
# BVD_pass = 'sZ8kxmMN'
# BVD_key = '2DZ6cae9f9cf630fea11ba35d89d672fa480'
# contentType = 'application/x-www-form-urlencoded; charset=UTF-8'
##################################

client = Algorithmia.client()

ssl_context = ssl.create_default_context()
ssl_context.set_ciphers('DEFAULT@SECLEVEL=1')

WORKING_DIR = "/tmp/working_dir/"

def apply(input):
    try:
        os.mkdir(WORKING_DIR)
    except OSError:
        # We can pass this, because the only failure case is if the directory exists
        pass
    # input = json.loads(input)
    counter = 1
    result_dict = {}

    values = input["search_list"]

    get_result = getBVDID(values)
    bvdID = get_result[0]
    score = get_result[1]
    if score > .9:
        print(bvdID)
        search_results = get_results(bvdID)
        pdf_item = generatePDF(input["search_list"]["Name"], search_results)
        
    else:
        pdf_item = generate_blank_PDF(values)
        
    key = "result_" + str(counter)
    result_dict[key] = pdf_item
    counter += 1
    return result_dict
        
def getBVDID(values):
    
    BVD_user = 'E&YUSws'
    BVD_pass = 'sZ8kxmMN'
    BVD_key = '2DZ6cae9f9cf630fea11ba35d89d672fa480'
    contentType = 'application/x-www-form-urlencoded; charset=UTF-8'
    
    data = urllib.parse.urlencode(values)
    data = data.encode('ascii') # data should be bytes

    headers = {'contentType': contentType, 'apitoken': BVD_key}
    url = 'https://webservices.bvdinfo.com/rest/orbis4/match'

    # try: 
    req = urllib.request.Request(url, data, headers)
    # req.set_proxy(proxy, 'http')
    # req.set_proxy(proxy, 'https')
    print(req)
    
    response = urllib.request.urlopen(req, context=ssl_context)
    print(response)
    results = response.read()

    results = json.loads(results.decode('utf-8'))[0]
    
    print(results)

    # print("Match Response " + str(x))
    # print(results)
    # print('\n')
#     print(results['BvDID'])

    search_bvdid = results['BvDID']
    score = results['Score']
    return search_bvdid, score
    # except: 
    #     print ("No results found.")
    #     return 0, 0

    
    #####################################################################################
    
def get_results(bvdID):
    
    BVD_user = 'E&YUSws'
    BVD_pass = 'sZ8kxmMN'
    BVD_key = '2DZ6cae9f9cf630fea11ba35d89d672fa480'
    contentType = 'application/x-www-form-urlencoded; charset=UTF-8'
    
    # query_file = open("BVD_Query_1.txt", "r+")
    # query_string = query_file.read()
    query_string = """DEFINE F1 AS [Filter.Name=ContactsFilter;ContactsFilter.IfHomeOnlyReturnCountry=1;ContactsFilter.Currents=True;ContactsFilter.CurrentPreviousQueryString=0;
        ContactsFilter.SourcesToExcludeQueryString=99B|59B|69B|70B|0|278;ContactsFilter.HierarchicCodeToExcludeQueryString=3|4;ContactsFilter.HierarchicCodeQueryString=0|1|2],
        F2 AS [Filter.Name=BeneficialOwnersFilter;BeneficialOwnersFilter.MinPercentBOFirstLevel=1000;BeneficialOwnersFilter.MinPercentBOHigherLevel=5001;BeneficialOwnersFilter.MinPercentBOLastLevelIndividual=1000;
        BeneficialOwnersFilter.AcceptNaPercentageAtLastLevelIndividual=True;BeneficialOwnersFilter.EjectIndividualsEvenIfAllWOUntilIndividualAndMinPercentBOFirstLevelIsOK=False;
        BeneficialOwnersFilter.KeepOnlyOnePathForEachBO_OUB=True;BeneficialOwnersFilter.BOFromRegisterOnly=False;],F3 AS [Filter.Name=BeneficialOwnersFilterDefinition10_10;
        BeneficialOwnersFilterDefinition10_10.MinPercentBOFirstLevel=1000;BeneficialOwnersFilterDefinition10_10.MinPercentBOHigherLevel=1000;BeneficialOwnersFilterDefinition10_10.MinPercentBOLastLevelIndividual=1000;
        BeneficialOwnersFilterDefinition10_10.AcceptNaPercentageAtLastLevelIndividual=True;BeneficialOwnersFilterDefinition10_10.EjectIndividualsEvenIfAllWOUntilIndividualAndMinPercentBOFirstLevelIsOK=False;
        BeneficialOwnersFilterDefinition10_10.KeepOnlyOnePathForEachBO_OUB=True;BeneficialOwnersFilterDefinition10_10.BOFromRegisterOnly=False;],F4 AS [Filter.Name=Subsidiaries;Subsidiaries.RemoveVessels=0;
        Subsidiaries.RemoveBranches=1;Subsidiaries.ControlShareholders=0;Subsidiaries.UltimatesIASOnlyEqU=1;Subsidiaries.QuotedShareholders=0;Subsidiaries.UltimatesIASOnlyDiffU=1;Subsidiaries.Ultimates=0;
        Subsidiaries.ListedIASDefinitionOnly=0;Subsidiaries.IsBvDLiensNote53=1;Subsidiaries.RecursionLevel=1;],F5 AS [Filter.Name=ContactsFilter;ContactsFilter.HierarchicCodeToExcludeQueryString=3|4;
        ContactsFilter.HierarchicCodeCountQueryString=0|1|2;ContactsFilter.SourcesToExcludeQueryString=99B|59B|69B|70B|0|278;ContactsFilter.IfHomeOnlyReturnCountry=1;ContactsFilter.HierarchicCodeToExcludeCountQueryString=3|4;
        ContactsFilter.HierarchicCodeQueryString=0|1|2;ContactsFilter.CurrentPreviousQueryString=0;ContactsFilter.Currents=True],F6 AS [Filter.Name=ContactsFilter;ContactsFilter.HierarchicCodeToExcludeQueryString=3|4;
        ContactsFilter.SourcesToExcludeQueryString=99B|59B|69B|70B|0|278;ContactsFilter.IfHomeOnlyReturnCountry=1;ContactsFilter.HierarchicCodeQueryString=0|1|2;ContactsFilter.CurrentPreviousQueryString=0;ContactsFilter.Currents=True],
        F7 AS [Filter.Name=ContactsFilter;ContactsFilter.HierarchicCodeToExcludeQueryString=3|4;ContactsFilter.HierarchicCodeCountQueryString=0|1|2;ContactsFilter.SourcesToExcludeQueryString=99B|59B|69B|70B|0|278;
        ContactsFilter.IfHomeOnlyReturnCountry=1;ContactsFilter.HierarchicCodeToExcludeCountQueryString=3|4;ContactsFilter.HierarchicCodeQueryString=0|1|2;ContactsFilter.CurrentPreviousQueryString=1;ContactsFilter.Previous=True],
        F8 AS [Filter.Name=ControllingShareholders;ControllingShareholders.RemoveVessels=1;ControllingShareholders.ControlShareholders=0;ControllingShareholders.UltimatesIASOnlyEqU=1;ControllingShareholders.UseBranchHeadQuarter=1;
        ControllingShareholders.IsBvDLiensNote53=1;ControllingShareholders.RemoveSubjectFromPathToGUO=1;ControllingShareholders.Ultimates=0;ControllingShareholders.ListedIASDefinitionOnly=0;ControllingShareholders.PathToUltimate=1;
        ControllingShareholders.QuotedShareholders=0;ControllingShareholders.UltimatesIASOnlyDiffU=1;],P1 AS [Parameters.RepeatingDimension=WEBSITE_COUNT],P2 AS [Parameters.RepeatingDimension=EMAIL_COUNT],
        P3 AS [Parameters.RepeatingDimension=PHONE_NUMBER_COUNT],P4 AS [Parameters.RepeatingDimension=NrOfBvDContacts],P5 AS [Parameters.RepeatingDimension=BO_COUNT],P6 AS [Parameters.RepeatingDimension=SUB_COUNT],
        P7 AS [Parameters.RepeatingDimension=NATIONAL_ID_COUNT],P8 AS [Parameters.RepeatingDimension=VAT_NUMBER_COUNT],P9 AS [Parameters.RepeatingDimension=TRADE_REGISTER_NUMBER_COUNT],
        P10 AS [Parameters.RepeatingDimension=OUB_COUNT],P11 AS [Parameters.DimensionSelName=BOI_COUNT;BOI_COUNT.Offset0=0;BOI_COUNT.Count0=-1;BOI_COUNT.Offset1=0;BOI_COUNT.Count1=-1;],
        P12 AS [Parameters.DimensionSelName=OUBI_COUNT;OUBI_COUNT.Offset0=0;OUBI_COUNT.Count0=-1;OUBI_COUNT.Offset1=0;OUBI_COUNT.Count1=-1;],P13 AS [Parameters.RepeatingDimension=CSH_COUNT],
        P14 AS [Parameters.RepeatingDimension=INDUSTRY_PRIMARY_CODE_COUNT],P15 AS [Parameters.RepeatingDimension=USSIC_PRIMARY_CODE_COUNT],P16 AS [Parameters.RepeatingDimension=STATUS_COUNT]; 
        SELECT LINE CONTACT_INFORMATION.ADDRESS_LINE1 AS ADDRESS_LINE1, LINE CONTACT_INFORMATION.ADDRESS_LINE2 AS ADDRESS_LINE2, LINE CONTACT_INFORMATION.POSTCODE AS POSTCODE, LINE CONTACT_INFORMATION.CITY AS CITY, 
        LINE CONTACT_INFORMATION.COUNTRY AS COUNTRY, LINE CONTACT_INFORMATION.COUNTRY_ISO_CODE AS COUNTRY_ISO_CODE, LINE CONTACT_INFORMATION.WEBSITE USING P1 AS WEBSITE, LINE CONTACT_INFORMATION.EMAIL USING P2 AS EMAIL, 
        LINE CONTACT_INFORMATION.PHONE_NUMBER USING P3 AS PHONE_NUMBER, LINE DMC_CONTACTS.CPYCONTACTS_HEADER_PotentialyInWocoFormatted FILTER F1 USING P4 AS CPYCONTACTS_HEADER_PotentialyInWocoFormatted, 
        LINE BO_INFO.BO_STATUSCOMPANY FILTER F2 AS BO_STATUS, LINE BO_INFO.BO_DEFINITION_10_10_COUNT FILTER F3 AS BO_DEFINITION_10_10_COUNT, LINE BO.BO_NAME FILTER F2 USING P5 AS BO_NAME, 
        LINE BO.BO_ENTITY_TYPE FILTER F2 USING P5 AS BO_ENTITY_TYPE, LINE BO.BO_BIRTHDATE FILTER F2 USING P5 AS BO_BIRTHDATE, LINE SUB.SUB_COUNT FILTER F4 AS SUB_COUNT, LINE SUB.SUB_NAME FILTER F4 USING P6 AS SUB_NAME, 
        LINE SUB.SUB_BVD_ID_NUMBER FILTER F4 USING P6 AS SUB_BVD_ID_NUMBER, LINE INDUSTRY_ACTIVITIES.PRODUCTS_SERVICES AS PRODUCTS_SERVICES, LINE OVERVIEW.OVERVIEW_PRIMARY_BUSINESS_LINE AS OVERVIEW_PRIMARY_BUSINESS_LINE, 
        LINE OVERVIEW.OVERVIEW_MAIN_ACTIVITY AS OVERVIEW_MAIN_ACTIVITY, LINE OVERVIEW.OVERVIEW_COUNTRY_NAME AS OVERVIEW_COUNTRY_NAME, LINE INDUSTRY_ACTIVITIES.BVD_SECTOR_CORE_LABEL AS BVD_SECTOR_CORE_LABEL, 
        LINE IDENTIFIERS.NATIONAL_ID USING P7 AS NATIONAL_ID, LINE IDENTIFIERS.VAT_NUMBER USING P8 AS VAT_NUMBER, LINE IDENTIFIERS.LEI AS LEI, LINE IDENTIFIERS.TRADE_REGISTER_NUMBER USING P9 AS TRADE_REGISTER_NUMBER, 
        LINE LEGAL_ACCOUNT_INFORMATION.NAME AS NAME, LINE DMC_CONTACTS.CPYCONTACTS_MEMBERSHIP_DIFFERENT_PERSONS_CNT1 FILTER F5 AS CPYCONTACTS_MEMBERSHIP_DIFFERENT_PERSONS_CNT, 
        LINE DMC_CONTACTS.CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred FILTER F6 USING P4 AS CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred, 
        LINE DMC_CONTACTS.CPYCONTACTS_MEMBERSHIP_OriginalJobTitle FILTER F6 USING P4 AS CPYCONTACTS_MEMBERSHIP_Function, LINE DMC_CONTACTS.CPYCONTACTS_MEMBERSHIP_DIFFERENT_PERSONS_CNT2 
        FILTER F7 AS CPYCONTACTS_MEMBERSHIP_DIFFERENT_PERSONS_CNT_1, LINE DMC_CONTACTS.CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred FILTER F6 USING P4 AS CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred_1, 
        LINE BO.BO_NAME FILTER F2 USING P5 AS BO_NAME_1, LINE BO.BO_UCI FILTER F2 USING P5 AS BO_UCI, LINE BO.BO_ENTITY_TYPE FILTER F2 USING P5 AS BO_ENTITY_TYPE_1, LINE BO.BOI_COUNT FILTER F2 USING P5 AS BOI_COUNT, 
        LINE OUB.OUB_NAME FILTER F2 USING P10 AS OUB_NAME, LINE OUB.OUB_COUNT FILTER F2 AS OUB_COUNT, LINE BOI.BOI_NAME FILTER F2 USING P11 AS BOI_NAME, LINE BOI.BOI_DIRECT_PCT FILTER F2 USING P11 AS BOI_DIRECT_PCT, 
        LINE BOI.BOI_TOTAL_PCT FILTER F2 USING P11 AS BOI_TOTAL_PCT, LINE OUBI.OUBI_NAME FILTER F2 USING P12 AS OUBI_NAME, LINE OUBI.OUBI_DIRECT_PCT FILTER F2 USING P12 AS OUBI_DIRECT_PCT, 
        LINE OUBI.OUBI_TOTAL_PCT FILTER F2 USING P12 AS OUBI_TOTAL_PCT, LINE CSH.CSH_NAME FILTER F8 USING P13 AS CSH_NAME, LINE CSH.CSH_DIRECT_PCT FILTER F8 USING P13 AS CSH_DIRECT_PCT, 
        LINE CSH.CSH_TOTAL_PCT FILTER F8 USING P13 AS CSH_TOTAL_PCT, LINE CSH.CSH_LEVEL FILTER F8 USING P13 AS CSH_LEVEL, LINE INDUSTRY_ACTIVITIES.INDUSTRY_PRIMARY_CODE USING P14 AS INDUSTRY_PRIMARY_CODE, 
        LINE INDUSTRY_ACTIVITIES.INDUSTRY_PRIMARY_LABEL USING P14 AS INDUSTRY_PRIMARY_LABEL, LINE INDUSTRY_ACTIVITIES.NACE2_MAIN_SECTION AS NACE2_MAIN_SECTION, LINE INDUSTRY_ACTIVITIES.NACE2_CORE_CODE AS NACE2_CORE_CODE, 
        LINE INDUSTRY_ACTIVITIES.NACE2_CORE_LABEL AS NACE2_CORE_LABEL, LINE INDUSTRY_ACTIVITIES.NAICS2017_CORE_CODE AS NAICS2017_CORE_CODE, LINE INDUSTRY_ACTIVITIES.NAICS2017_CORE_LABEL AS NAICS2017_CORE_LABEL, 
        LINE INDUSTRY_ACTIVITIES.USSIC_CORE_CODE AS USSIC_CORE_CODE, LINE INDUSTRY_ACTIVITIES.USSIC_CORE_LABEL AS USSIC_CORE_LABEL, LINE INDUSTRY_ACTIVITIES.USSIC_PRIMARY_CODE USING P15 AS USSIC_PRIMARY_CODE, 
        LINE INDUSTRY_ACTIVITIES.USSIC_PRIMARY_LABEL USING P15 AS USSIC_PRIMARY_LABEL, LINE LEGAL_ACCOUNT_INFORMATION.INFORMATION_PROVIDER AS INFORMATION_PROVIDER, LINE LEGAL_ACCOUNT_INFORMATION.COMPANY_CATEGORY AS COMPANY_CATEGORY, 
        LINE LEGAL_ACCOUNT_INFORMATION.STATUS USING P16 AS STATUS, LINE LEGAL_ACCOUNT_INFORMATION.INCORPORATION_DATE AS INCORPORATION_DATE, LINE LEGAL_ACCOUNT_INFORMATION.INCORPORATION_STATE AS INCORPORATION_STATE, 
        LINE LEGAL_ACCOUNT_INFORMATION.ENTITY_TYPE AS ENTITY_TYPE FROM RemoteAccess.UNIVERSAL"""
        
    url = 'https://webservices.bvdinfo.com/rest/orbis4/getdata'
    # values = {"bvdid": search_bvdid, "querystring": query_string}
    values = {"BvDIds": bvdID, "QueryString": query_string}


    data = urllib.parse.urlencode(values)
    data = data.encode('ascii') # data should be bytes

    contentType = 'application/x-www-form-urlencoded; charset=UTF-8'

    headers = {'contentType': contentType, 'apitoken': BVD_key}

    req = urllib.request.Request(url, data, headers)
    # req.set_proxy(proxy, 'http')
    # req.set_proxy(proxy, 'https')

    response = urllib.request.urlopen(req, context=ssl_context)
    results = response.read()
    results = json.loads(results.decode('utf-8'))[0]
    return(results)
    
    #     print(results)
    ######################################################################################
def generatePDF(search_name, results):
    
    # output_path = "data://.algo/mshterk/BVD_Search/temp/"
    curr_time = str(time.strftime("%Y-%m-%d-%H-%M-%S"))
    curr_time_doc = str(time.strftime("%Y-%m-%d %H:%M:%S"))

    name = str(results['NAME'])
    addr = str(results['ADDRESS_LINE1']) + ", " + str(results['CITY']) + ", " + str(results['COUNTRY']) + " " + str(results['POSTCODE'])
    website = str(results['WEBSITE'])
    email = str(results['EMAIL'])
    # cpy_names = str(results['CPY_NAMES'])
    bo_status = str(results['BO_STATUS'])
    bo_name = str(results['BO_NAME'])
    sub_count = str(results['SUB_COUNT'])
    sub_name = str(results['SUB_NAME'])

    pdf_fname = WORKING_DIR + str(name) + "_" + curr_time + '.pdf'

    styles = getSampleStyleSheet()
    doc = SimpleDocTemplate(pdf_fname, leftMargin = 0.75*inch, rightMargin = 0.75*inch, topMargin = 1*inch, bottomMargin = 1*inch)
    
    Story=[]
    
    ###Heading for overall report###
    h0 = Paragraph('KYC Profile', styles['Title'])
    Story.append(h0)

    p0=Paragraph("<para align=left><br/><font size=11><b><u>Entity Searched</u></b></font>: " + search_name + "</para>", styles['Heading2']) 
    Story.append(p0)
    
    p1=Paragraph("<para align=left><br/><font size=11><b><u>Entity Matched</u></b></font>: " + name + "</para>", styles['Heading2']) 
    Story.append(p1)

    p2=Paragraph("<para align=left><br/><font size=11><b><u>Date Searched</u></b></font>: " + curr_time_doc + "</para>", styles['Heading2']) 
    Story.append(p2)

    ###CONTACT INFO###
    p2_2 = Paragraph("<para align=left><br/><font size=11><b><u>Contact Information</u></b></font></para>", styles['Heading3'])
    Story.append(p2_2)

    p4_0 = Paragraph("<para align=center><b>Field</b></para>",styles['Normal'])
    p4_1 = Paragraph("<para align=center>Entity Name</para>",styles['Normal'])
    p4_2 = Paragraph("<para align=center>Address</para>",styles['Normal'])
    p4_3 = Paragraph("<para align=center>Website</para>",styles['Normal'])
    p4_4 = Paragraph("<para align=center>Email</para>",styles['Normal'])
    p4_5 = Paragraph("<para align=center>Phone Number</para>",styles['Normal'])
    p5_0 = Paragraph("<para align=center><b>Value </b></para>",styles['Normal'])
    p5_1 = Paragraph("<para align=center>" + str(name) + '</para>', styles['Normal'])
    p5_2 = Paragraph("<para align=center>" + str(addr) + '</para>', styles['Normal'])
    p5_3 = Paragraph("<para align=center>" + str(results['WEBSITE']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p5_4 = Paragraph("<para align=center>" + str(results['EMAIL']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p5_5 = Paragraph("<para align=center>" + str(results['PHONE_NUMBER']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    data1= [[p4_0, p5_0],
            [p4_1, p5_1],
            [p4_2, p5_2],
            [p4_3, p5_3],
            [p4_4, p5_4],
            [p4_5, p5_5]]


    t1=Table(data1, hAlign='CENTER', vAlign='CENTER')
    t1.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))

    Story.append(t1)
    ###END CONTACT INFO###

    ###LEGAL INFO###
    p6_1 = Paragraph("<para align=left><br/><font size=11><b><u>Legal Information</u></b></font></para>", styles['Heading3'])
    Story.append(p6_1)
    p7_0 = Paragraph("<para align=center><b>Field</b></para>",styles['Normal'])
    p7_1 = Paragraph("<para align=center>Company Category</para>",styles['Normal'])
    p7_2 = Paragraph("<para align=center>Status</para>",styles['Normal'])
    p7_3 = Paragraph("<para align=center>Incorporation Date</para>",styles['Normal'])
    p7_4 = Paragraph("<para align=center>Incorporation State (if US)</para>",styles['Normal'])
    p7_5 = Paragraph("<para align=center>Entity Type</para>",styles['Normal'])
    p7_6 = Paragraph("<para align=center>National ID</para>",styles['Normal'])
    p7_7 = Paragraph("<para align=center>VAT Number</para>",styles['Normal'])
    p7_8 = Paragraph("<para align=center>LEI</para>",styles['Normal'])
    p7_9 = Paragraph("<para align=center>Trade Register Number</para>",styles['Normal'])
    p8_0 = Paragraph("<para align=center><b>Value </b></para>",styles['Normal'])
    p8_1 = Paragraph("<para align=center>" + str(results['COMPANY_CATEGORY']) + '</para>', styles['Normal'])
    p8_2 = Paragraph("<para align=center>" + str(results['STATUS']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p8_3 = Paragraph("<para align=center>" + str(results['INCORPORATION_DATE']) + '</para>', styles['Normal'])
    p8_4 = Paragraph("<para align=center>" + str(results['INCORPORATION_STATE']) + '</para>', styles['Normal'])
    p8_5 = Paragraph("<para align=center>" + str(results['ENTITY_TYPE']) + '</para>', styles['Normal'])
    p8_6 = Paragraph("<para align=center>" + str(results['NATIONAL_ID']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p8_7 = Paragraph("<para align=center>" + str(results['VAT_NUMBER']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p8_8 = Paragraph("<para align=center>" + str(results['LEI']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p8_9 = Paragraph("<para align=center>" + str(results['TRADE_REGISTER_NUMBER']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])

    data3= [[p7_0, p8_0],
            [p7_1, p8_1],
            [p7_2, p8_2],
            [p7_3, p8_3],
            [p7_4, p8_4],
            [p7_5, p8_5],
            [p7_6, p8_6],
            [p7_7, p8_7],
            [p7_8, p8_8],
            [p7_9, p8_9]]

    t3=Table(data3, hAlign='CENTER', vAlign='CENTER')
    t3.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))
    Story.append(t3)
    ###END LEGAL INFO###

    ###INDUSTRY AND ACTIVITIES
    p9_1 = Paragraph("<para align=left><br/><font size=11><b><u>Industry & Activities</u></b></font></para>", styles['Heading3'])
    Story.append(p9_1)
    p10_0 = Paragraph("<para align=center><b>Field</b></para>",styles['Normal'])
    p10_1 = Paragraph("<para align=center>Sector</para>",styles['Normal'])
    p10_2 = Paragraph("<para align=center>Products and Services</para>",styles['Normal'])
    p10_3 = Paragraph("<para align=center>Industry Label</para>",styles['Normal'])
    p10_4 = Paragraph("<para align=center>NAICS2017 Core Code</para>",styles['Normal'])
    p10_5 = Paragraph("<para align=center>NAICS2017 Core Code Label</para>",styles['Normal'])
    p10_6 = Paragraph("<para align=center>USSIC Core Code</para>",styles['Normal'])
    p10_7 = Paragraph("<para align=center>USSIC Core Code Label</para>",styles['Normal'])
    p11_0 = Paragraph("<para align=center><b>Value </b></para>",styles['Normal'])
    p11_1 = Paragraph("<para align=center>" + str(results['BVD_SECTOR_CORE_LABEL']) + '</para>', styles['Normal'])
    p11_2 = Paragraph("<para align=center>" + str(results['PRODUCTS_SERVICES']) + '</para>', styles['Normal'])
    p11_3 = Paragraph("<para align=center>" + str(results['INDUSTRY_PRIMARY_LABEL']).replace('[', '').replace(']', '').replace("'", '') + '</para>', styles['Normal'])
    p11_4 = Paragraph("<para align=center>" + str(results['NAICS2017_CORE_CODE']) + '</para>', styles['Normal'])
    p11_5 = Paragraph("<para align=center>" + str(results['NAICS2017_CORE_LABEL']) + '</para>', styles['Normal'])
    p11_6 = Paragraph("<para align=center>" + str(results['USSIC_CORE_CODE']) + '</para>', styles['Normal'])
    p11_7 = Paragraph("<para align=center>" + str(results['USSIC_CORE_LABEL']) + '</para>', styles['Normal'])
    data4= [[p10_0, p11_0],
            [p10_1, p11_1],
            [p10_2, p11_2],
            [p10_3, p11_3],
            [p10_4, p11_4],
            [p10_5, p11_5],
            [p10_6, p11_6],
            [p10_7, p11_7]]

    t4=Table(data4, hAlign='CENTER', vAlign='CENTER')
    t4.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))

    Story.append(t4)

    ###END INDUSTRY AND ACTIVITIES###

    ###CURRENT DIRECTORS###

    counter = 0
    
    if results['CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred'] is None:
        pass
    else:
        tup1 = list(map(lambda c, y: (c,y),results['CPYCONTACTS_HEADER_FullNameOriginalLanguagePreferred'], results['CPYCONTACTS_MEMBERSHIP_Function']))
        grouped = []
        for key, group in groupby(tup1, lambda x: x[0]):
            grouped.append(list(group))
        dirs = [(x[0][0], [y[-1] for y in x]) for x in grouped]
    
        dirs_names = [i[0] for i in dirs]
        dirs_pos = [i[1] for i in dirs]
        p12_1 = Paragraph("<para align=left><br/><font size=11><b><u>Directors*</u></b></font></para>", styles['Heading3'])
        Story.append(p12_1)
    
        table_header_L = Paragraph("<para align=center><b>Name</b></para>", styles['Normal'])
        table_header_M = Paragraph("<para align=center><b>Position</b></para>", styles['Normal'])
    
        row_list = [[table_header_L, table_header_M]]
        if len(dirs) < 20:
            while counter < len(dirs):
                t_l = Paragraph("<para align=center>" + str(dirs_names[counter]) + "</para>", styles['Normal'])
                t_r = Paragraph("<para align=center>" + str(dirs_pos[counter]).replace('[', '').replace(']', '').replace("'", '') + "</para>", styles['Normal'])
                row_list.append([t_l, t_r])
                counter = counter + 1
        else:
            while counter < 20:
                t_l = Paragraph("<para align=center>" + str(dirs_names[counter]) + "</para>", styles['Normal'])
                t_r = Paragraph("<para align=center>" + str(dirs_pos[counter]).replace('[', '').replace(']', '').replace("'", '') + "</para>", styles['Normal'])
                row_list.append([t_l, t_r])
                counter = counter + 1
    
        t5=Table(row_list, hAlign='CENTER', vAlign='CENTER')
        t5.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                               ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                               ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))
    
        Story.append(t5)
        
        
        style_footer = ParagraphStyle(name='right', parent=styles['Normal'], fontName='Helvetica',
                fontSize=9)
    
        footer0 = Paragraph("*Top 20 Directors shown, as applicable", style_footer)

        Story.append(footer0)

    ###End Current Directors###

    ###Beneficial Ownership###
    p13_1 = Paragraph("<para align=left><br/><font size=11><b><u>Beneficial Ownership Information</u></b></font></para>", styles['Heading3'])
    Story.append(p13_1)

    counter = 0

    table_header_L = Paragraph("<para align=center><b>Beneficial Owner Name</b></para>", styles['Normal'])
    table_header_M = Paragraph("<para align=center><b>Beneficial Owner Entity Type</b></para>", styles['Normal'])

    row_list = [[table_header_L, table_header_M]]

    if results['BO_NAME_1'] is None:
        t_l = Paragraph("<para align=center>" + str(results['BO_NAME_1']) + "</para>", styles['Normal'])
        t_r = Paragraph("<para align=center>" + str(results['BO_ENTITY_TYPE_1']) + "</para>", styles['Normal'])
        row_list.append([t_l, t_r])

    else:
        while counter < len(results['BO_NAME_1']):
            t_l = Paragraph("<para align=center>" + str(results['BO_NAME_1'][0][counter]) + "</para>", styles['Normal'])
            t_r = Paragraph("<para align=center>" + str(results['BO_ENTITY_TYPE_1'][0][counter]) + "</para>", styles['Normal'])
            row_list.append([t_l, t_r])
            counter += 1

    t6=Table(row_list, hAlign='CENTER', vAlign='CENTER')
    t6.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))

    Story.append(t6) 

    p14_1 = Paragraph("<para align=left><br/><font size=11><b><u>Beneficial Owner Intermediary</u></b></font></para>", styles['Heading3'])
    Story.append(p14_1)

    counter = 0

    table_header_L = Paragraph("<para align=center><b>Beneficial Owner Intermediary</b></para>", styles['Normal'])
    table_header_M = Paragraph("<para align=center><b>Beneficial Owner Direct Ownership %</b></para>", styles['Normal'])
    table_header_R = Paragraph("<para align=center><b>Beneficial Owner Total Ownership %</b></para>", styles['Normal'])                    

    row_list = [[table_header_L, table_header_M, table_header_R]]
    if results['OUBI_NAME'] is None:
        t_l = Paragraph("<para align=center>" + str(results['OUBI_NAME']) + "</para>", styles['Normal'])
        t_m = Paragraph("<para align=center>" + str(results['OUBI_DIRECT_PCT']) + "</para>", styles['Normal'])
        t_r = Paragraph("<para align=center>" + str(results['OUBI_TOTAL_PCT']) + "</para>", styles['Normal'])
        row_list.append([t_l, t_m, t_r])
    else:
        while counter < len(results['OUBI_NAME'][0]):
            t_l = Paragraph("<para align=center>" + str(results['OUBI_NAME'][0][counter]) + "</para>", styles['Normal'])
            t_m = Paragraph("<para align=center>" + str(results['OUBI_DIRECT_PCT'][0][counter]) + "</para>", styles['Normal'])
            t_r = Paragraph("<para align=center>" + str(results['OUBI_TOTAL_PCT'][0][counter]) + "</para>", styles['Normal'])
            row_list.append([t_l, t_m, t_r])
            counter += 1

    t7=Table(row_list, hAlign='CENTER', vAlign='CENTER')
    t7.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))

    Story.append(t7) 

    p15_1 = Paragraph("<para align=left><br/><font size=11><b><u>Controlling Shareholders</u></b></font></para>", styles['Heading3'])
    Story.append(p15_1)

    counter = 0

    table_header_L = Paragraph("<para align=center><b>Controlling Shareholder</b></para>", styles['Normal'])
    table_header_M = Paragraph("<para align=center><b>Controlling Shareholder Direct Ownership %</b></para>", styles['Normal'])
    table_header_R = Paragraph("<para align=center><b>Controlling Shareholder Total Ownership %</b></para>", styles['Normal'])                    
    table_header_R1 = Paragraph("<para align=center><b>Controlling Shareholder Step</b></para>", styles['Normal'])

    row_list = [[table_header_L, table_header_M, table_header_R, table_header_R1]]
    if results['CSH_NAME'] is None:
        t_l = Paragraph("<para align=center>" + str(results['CSH_NAME']) + "</para>", styles['Normal'])
        t_m = Paragraph("<para align=center>" + str(results['CSH_DIRECT_PCT']) + "</para>", styles['Normal'])
        t_r = Paragraph("<para align=center>" + str(results['CSH_TOTAL_PCT']) + "</para>", styles['Normal'])
        t_r1 = Paragraph("<para align=center>" + str(results['CSH_LEVEL']) + "</para>", styles['Normal'])
        row_list.append([t_l, t_m, t_r, t_r1])
    else:
        while counter < len(results['CSH_NAME']):
            t_l = Paragraph("<para align=center>" + str(results['CSH_NAME'][counter]) + "</para>", styles['Normal'])
            t_m = Paragraph("<para align=center>" + str(results['CSH_DIRECT_PCT'][counter]) + "</para>", styles['Normal'])
            t_r = Paragraph("<para align=center>" + str(results['CSH_TOTAL_PCT'][counter]) + "</para>", styles['Normal'])
            t_r1 = Paragraph("<para align=center>" + str(results['CSH_LEVEL'][counter]) + "</para>", styles['Normal'])
            row_list.append([t_l, t_m, t_r, t_r1])
            counter += 1

    t8=Table(row_list, hAlign='CENTER', vAlign='CENTER')
    t8.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
                           ('BOX', (0,0), (-1,-1), 0.25, colors.black),
                           ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))

    Story.append(t8)

    p39=Paragraph("<para align=center><font size=11><b><u>Notes </u></b></font><font size=8></font> </para>", styles['Heading3'])
    Story.append(p39)

    style_footer = ParagraphStyle(name='right', parent=styles['Normal'], fontName='Helvetica',
                fontSize=9)
    
    footer0 = Paragraph("*Top 20 Directors shown, as applicable", style_footer)

    Story.append(footer0)
    
    footer1 = Paragraph("*C: Corporate", style_footer)

    Story.append(footer1)

    footer2 = Paragraph("*WO: Wholly Owned", style_footer)

    Story.append(footer2)
    


    doc.build(Story)

    outfile = "data://.my/bvc/KYC_Profile_" + str(name) + "_" + curr_time + '.pdf'
    # tempfile_PDF = WORKING_DIR + str(name) + "_" + curr_time + '.pdf'
    
    client.file(outfile).putFile(pdf_fname)
    
    #outfile = outfile.replace("data://.algo/temp/", "data://.algo/mshterk/BVD_Search/temp/")
    
    return1 = True
    
    return {"path": outfile, "results": results, "results_returned": return1}

def generate_blank_PDF(values):
    name = values['Name']
    curr_time = str(time.strftime("%Y-%m-%d-%H-%M-%S"))
    curr_time_doc = str(time.strftime("%Y-%m-%d %H:%M:%S"))
    pdf_fname = WORKING_DIR + str(name) + "_" + curr_time + '.pdf'

    styles = getSampleStyleSheet()
    doc = SimpleDocTemplate(pdf_fname, leftMargin = 0.75*inch, rightMargin = 0.75*inch, topMargin = 1*inch, bottomMargin = 1*inch)
    
    Story=[]
    
    ###Heading for overall report###
    h0 = Paragraph('KYC Profile', styles['Title'])
    Story.append(h0)

    p0=Paragraph("<para align=left><br/><font size=11><b><u>Entity Searched</u></b></font>: " + name + "</para>", styles['Heading2']) 
    Story.append(p0)
    
    p1=Paragraph("<para align=left><br/><font size=11><b><u>Entity Matched</u></b></font>: " + "NONE" + "</para>", styles['Heading2']) 
    Story.append(p1)

    p2=Paragraph("<para align=left><br/><font size=11><b><u>Date Searched</u></b></font>: " + curr_time_doc + "</para>", styles['Heading2']) 
    Story.append(p2)
    
    p3 = Paragraph("<para align=left><br/><font size=11><b><u>No profiles found exceeding name matching threshold.</u></b></font></para>", styles['Heading2']) 
    Story.append(p3)
    
    # keys = list(values.keys())

    # table_header_L = Paragraph("<para align=center><b>Search Terms</b></para>", styles['Normal'])
    # table_header_M = Paragraph("<para align=center><b>Values</b></para>", styles['Normal'])
    # row_list = [[table_header_L, table_header_M]]
    # for x in range(len(values)):
    #     t_l = Paragraph("<para align=center>" + str(keys[x]) + "</para>", styles['Normal'])
    #     t_m = Paragraph("<para align=center>" + str(values[keys[x]]) + "</para>", styles['Normal'])
    # row_list.append([t_l, t_m])
    
    # t1=Table(row_list, hAlign='CENTER', vAlign='CENTER')
    # t1.setStyle(TableStyle([('INNERGRID', (0,0), (-1,-1), 0.25, colors.black),
    #                       ('BOX', (0,0), (-1,-1), 0.25, colors.black),
    #                       ('VALIGN', (0, 0), (-1, -1), 'MIDDLE')]))
    # Story.append(t1)
    
    doc.build(Story)
    
    outfile = "data://.algo/mshterk/BVD_Search/temp/KYC_Profile_" + str(name) + "_" + curr_time + '.pdf'
    # tempfile_PDF = WORKING_DIR + str(name) + "_" + curr_time + '.pdf'
    
    client.file(outfile).putFile(pdf_fname)
    
    #outfile = outfile.replace("data://.algo/temp/", "data://.algo/mshterk/BVD_Search/temp/")
    
    return1 = False
    
    return {"path": outfile, "results": "None", "results_returned": return1}
