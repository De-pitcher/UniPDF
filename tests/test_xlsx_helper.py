import zipfile
import sys
import os

def create_simple_xlsx(filename):
    # Minimal standard OpenXML spreadsheet structure
    content_types = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>"""

    root_rels = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"""

    workbook_xml = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="Financials" sheetId="1" r:id="rId1"/>
  </sheets>
</workbook>"""

    workbook_rels = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
</Relationships>"""

    sheet1_xml = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    <row r="1">
      <c r="A1" t="inlineStr"><is><t>Item Code</t></is></c>
      <c r="B1" t="inlineStr"><is><t>Description</t></is></c>
      <c r="C1" t="inlineStr"><is><t>Units</t></is></c>
      <c r="D1" t="inlineStr"><is><t>Unit Price ($)</t></is></c>
      <c r="E1" t="inlineStr"><is><t>Total Amount ($)</t></is></c>
    </row>
    <row r="2">
      <c r="A2" t="inlineStr"><is><t>ITM-001</t></is></c>
      <c r="B2" t="inlineStr"><is><t>High-Performance Cloud Compute Node</t></is></c>
      <c r="C2"><v>12</v></c>
      <c r="D2"><v>249.50</v></c>
      <c r="E2"><v>2994.00</v></c>
    </row>
    <row r="3">
      <c r="A3" t="inlineStr"><is><t>ITM-002</t></is></c>
      <c r="B3" t="inlineStr"><is><t>Managed Redis In-Memory Cluster</t></is></c>
      <c r="C3"><v>4</v></c>
      <c r="D3"><v>85.00</v></c>
      <c r="E3"><v>340.00</v></c>
    </row>
    <row r="4">
      <c r="A4" t="inlineStr"><is><t>ITM-003</t></is></c>
      <c r="B4" t="inlineStr"><is><t>Edge CDN Distribution &amp; SSL</t></is></c>
      <c r="C4"><v>1</v></c>
      <c r="D4"><v>120.00</v></c>
      <c r="E4"><v>120.00</v></c>
    </row>
  </sheetData>
</worksheet>"""

    with zipfile.ZipFile(filename, 'w', zipfile.ZIP_DEFLATED) as zf:
        zf.writestr('[Content_Types].xml', content_types)
        zf.writestr('_rels/.rels', root_rels)
        zf.writestr('xl/workbook.xml', workbook_xml)
        zf.writestr('xl/_rels/workbook.xml.rels', workbook_rels)
        zf.writestr('xl/worksheets/sheet1.xml', sheet1_xml)

def create_complex_xlsx(filename):
    content_types = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
  <Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>"""

    root_rels = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"""

    workbook_xml = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="Engineering Metrics" sheetId="1" r:id="rId1"/>
    <sheet name="Audit Log" sheetId="2" r:id="rId2"/>
  </sheets>
</workbook>"""

    workbook_rels = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
</Relationships>"""

    # Sheet 1: 30 rows
    rows_xml = []
    rows_xml.append("""<row r="1">
      <c r="A1" t="inlineStr"><is><t>Service Name</t></is></c>
      <c r="B1" t="inlineStr"><is><t>P99 Latency (ms)</t></is></c>
      <c r="C1" t="inlineStr"><is><t>RPS</t></is></c>
      <c r="D1" t="inlineStr"><is><t>Error Rate (%)</t></is></c>
      <c r="E1" t="inlineStr"><is><t>Health Status</t></is></c>
    </row>""")

    services = ["Auth-Gateway", "User-Service", "Billing-Worker", "Sync-Orchestrator", "Audit-Logger", "Report-Generator"]
    for i in range(2, 32):
        svc = services[(i - 2) % len(services)]
        latency = 15.2 + ((i * 3) % 40)
        rps = 1500 + (i * 24)
        err = 0.02 * (i % 5)
        rows_xml.append(f"""<row r="{i}">
      <c r="A{i}" t="inlineStr"><is><t>{svc}-{i:02d}</t></is></c>
      <c r="B{i}"><v>{latency:.2f}</v></c>
      <c r="C{i}"><v>{rps}</v></c>
      <c r="D{i}"><v>{err:.3f}</v></c>
      <c r="E{i}" t="inlineStr"><is><t>HEALTHY</t></is></c>
    </row>""")

    sheet1_xml = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    {"".join(rows_xml)}
  </sheetData>
</worksheet>"""

    # Sheet 2: Audit log
    sheet2_xml = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    <row r="1">
      <c r="A1" t="inlineStr"><is><t>Timestamp</t></is></c>
      <c r="B1" t="inlineStr"><is><t>Principal</t></is></c>
      <c r="C1" t="inlineStr"><is><t>Action</t></is></c>
      <c r="D1" t="inlineStr"><is><t>Result Code</t></is></c>
    </row>
    <row r="2">
      <c r="A2" t="inlineStr"><is><t>2026-09-10 03:00:00</t></is></c>
      <c r="B2" t="inlineStr"><is><t>deployer@system.local</t></is></c>
      <c r="C2" t="inlineStr"><is><t>PROMOTION_DEV_TO_MAIN</t></is></c>
      <c r="D2"><v>200</v></c>
    </row>
  </sheetData>
</worksheet>"""

    with zipfile.ZipFile(filename, 'w', zipfile.ZIP_DEFLATED) as zf:
        zf.writestr('[Content_Types].xml', content_types)
        zf.writestr('_rels/.rels', root_rels)
        zf.writestr('xl/workbook.xml', workbook_xml)
        zf.writestr('xl/_rels/workbook.xml.rels', workbook_rels)
        zf.writestr('xl/worksheets/sheet1.xml', sheet1_xml)
        zf.writestr('xl/worksheets/sheet2.xml', sheet2_xml)

if __name__ == '__main__':
    os.makedirs('test_files', exist_ok=True)
    create_simple_xlsx('test_files/sample.xlsx')
    create_complex_xlsx('test_files/complex_stress_test.xlsx')
    print("XLSX test fixtures generated successfully!")
