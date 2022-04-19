
from lib import db

from argparse import ArgumentParser
from tqdm import tqdm
import datetime
import os
import re
import xlwings

year_folder = re.compile(r"\d{4}")
TS = r"\\hssfileserv1\hssshared\hssi lean\cad-cam\tagschedule"

bad_date_pattern = re.compile(r"(\d{1,2})/(\d{1,2})/2?(\d{2})/?")

HARVEST_SHEETS = ["WEBS", "FLANGES", "PARTS"]

class ProgramDataHarvester:

    def __init__(self):
        self.app = None
        self.wb = None

        self.data = dict()
        self.data_changed_since_last_dump = False

        parser = ArgumentParser()
        parser.add_argument("--year")
        parser.add_argument("--upload")
        
        self.args = parser.parse_args()

        self.data_file = 'data\\prog_data_map_{}.txt'.format(self.args.year or "all")

        if self.args.upload:
            pass
        else:
            self.run()

    def run(self):
        if len(xlwings.apps) > 0 and not self.args.year:
            self.app = xlwings.apps.active
        else:
            self.app = xlwings.App()


        try:
            for d in os.scandir(TS):
                
                if d.is_dir() and year_folder.match(d.name):
                    if self.args.year and d.name != self.args.year:
                        continue

                    self.data = dict()
                    self.data_file = 'data\\prog_data_map_{}.txt'.format(d.name)
                    self.load_data()
                    
                    for f in tqdm(os.scandir(d.path), desc=d.name, total=len(os.listdir(d.path))):
                        if f.name.endswith(".xls"):
                            self.harvest_wb(f.path)

                            self.dump_data()

        except Exception as e:
            print("Run error: " + str(e))

        self.app.quit()

    def load_data(self):
        if not os.path.exists(self.data_file):
            return

        with open(self.data_file, 'r') as f:
            for line in f.readlines():
                prog, checked, printed, remarks = line.strip("\n").split(",")
                self.data[prog] = ProgramData()
                self.data[prog].update_from_txt(checked, printed, remarks)

    def dump_data(self):
        if not self.data_changed_since_last_dump:
            return

        with open(self.data_file, 'w') as out:
            for prog in sorted(self.data):
                out.write(",".join([prog, *self.data[prog].vals()]) + "\n")

        self.data_changed_since_last_dump = False

    def harvest_wb(self, filename):
        self.wb = xlwings.Book(filename, update_links=False, read_only=True)
        for s in self.wb.sheets:
            if s.name in HARVEST_SHEETS:
                self.harvest_sheet(s)

        self.wb.close()
        self.wb = None


    def harvest_sheet(self, name):
        try:
            sheet = self.wb.sheets[name]

            def get_index(*vals, error_on_fail=True):
                for row in sheet.range("A1:AZ2").value:
                    for val in vals:
                        try:
                            return row.index(val) + 1
                        except ValueError:
                            pass

                if error_on_fail:
                    raise ValueError("{} not found in header".format(val))
                
                return 1

            try:
                program_col = get_index("Prog #", "Program #")
                checked_col = get_index("Checked", error_on_fail=False)
                printed_col = get_index("Date Printed")
                remarks_col = get_index("Remarks\n(Slab Weld, strip, etc)", error_on_fail=False)

            except ValueError:
                print("\nFailed to parse header for {} [{}]".format(self.wb.name, name))
                return

            i = 4
            while sheet.range(i, 3).value:
                program = sheet.range(i, program_col).value
                checked = sheet.range(i, checked_col).value

                if program and checked:
                    if name == 'PARTS':
                        printed = remarks = None
                    else:
                        printed = sheet.range(i, printed_col).value
                        remarks = sheet.range(i, remarks_col).value

                    if type(program) is float:
                        program = "{:g}".format(program)

                    if program not in self.data:
                        self.data[program] = ProgramData()

                    self.data[program].update(checked, printed, remarks)

                    self.data_changed_since_last_dump = True

                i += 1

        except Exception as e:
            print("Error: " + str(e))


class ProgramData:

    def __init__(self):
        self.checked = None
        self.printed = None
        self.remarks = []
        
        self.non_date_printed = []

    def update(self, checked, printed, remarks):
        self.checked = str(checked)

        # get max print date
        if printed:
            if type(printed) is not datetime.datetime:
                if type(printed) is str and bad_date_pattern.match(printed):
                    month, day, year = bad_date_pattern.match(printed).groups()
                    year = "20" + year

                    return self.update(checked, datetime.datetime(int(year), int(month), int(day)), remarks)

                self.non_date_printed.append(printed)
            elif self.printed and self.printed != printed:
                self.printed = max(self.printed, printed)
            else:
                self.printed = printed

        # append remarks
        if remarks:
            self.remarks.append(str(remarks))

    def update_from_txt(self, checked, printed, remarks):
        self.checked = checked
        
        if ";" in printed:
            printed, non_p = printed.split(";")
            self.non_date_printed = non_p.split("*")
        self.printed = datetime.fromisoformat(printed)
        self.remarks = list(remarks.split(";"))
    
    def vals(self):

        p = ''
        if self.printed:
            p = str(self.printed.date())
        if self.non_date_printed:
            p = "{};{}".format(p, "*".join(set(self.non_date_printed)))

        return self.checked, str(p), ";".join(set(self.remarks))


if __name__ == "__main__":
    ProgramDataHarvester()
