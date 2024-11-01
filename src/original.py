import re
from zipfile import ZipFile, BadZipFile
from pathlib import Path
import argparse
from collections import defaultdict
import os 
from tqdm import tqdm
from multiprocessing import Pool
from collections import Counter

class Object:
    def __init__(self, name, id, types, pilot, creation, coord, munition=False):
        self.name = name
        self.id = id
        self.types = types
        self.creation = creation
        self.children = []
        self.parent = None
        self.pilot = pilot
        self.coord = coord
        self.alive = True
        self.munition = munition
        self.destroy_time = None

class Coord:
    def __init__(self, long, lat, alt):
        self.long = float(long)
        self.lat = float(lat)
        self.alt = float(alt)

    def __str__(self):
        return(f"{self.long}  {self.lat}  {self.alt}")

    def close(self, coord):
        return False
    
    def dist(self, coord):
        return (coord.long - self.long)**2 + (coord.lat - self.lat)**2 + (coord.alt - self.alt)**2

def extract_zip(path):
    if ".zip" in path.suffixes:
        try:
            with ZipFile(path) as myzip:
                try:
                    with myzip.open(os.path.basename(str(path)).replace("zip", "txt")) as myfile:
                        text = myfile.read()
                except:
                    with myzip.open(myzip.filelist[0]) as myfile:
                        text = myfile.read()
        except BadZipFile:
            #print("EXCEPTION with ", path)
            return None
    elif ".txt" in path.suffixes:
        f = open(path, "r", encoding="utf-8")
        text = f.read()
    text = text.decode("utf-8").splitlines() 
    assert("\ufeffFileType=text/acmi/tacview" in text[0])
    return text

def parse_tacview(tac):
    
    objects = {}
    missiles = []
    object_pattern = r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+),Pilot=([\w+\- \|]+)"
    missile_pattern = r"^([0-9a-f]+),T=([0-9\.-]+)\|([0-9\.-]+)\|([0-9\.-]+)[0-9\.|-]+,Type=([\w+]+),Name=([\w+\- \._]+)"
    update_pattern = r"^([0-9a-f]+),T=([0-9\.-]*)\|([0-9\.-]*)\|([0-9\.-]*)[0-9\.|-]+"
    destruction_pattern = r"-([0-9a-f]+)$"

    last_time = 0
    tracked_item = 0x601
    start = 0
    for line in tac:
        if line.startswith("#"):
            last_time = float(line[1:])
            start = 1
        elif start == 0:
            continue
        elif ",Pilot=" in line and re.match(object_pattern, line, re.UNICODE):
            matches = re.findall(object_pattern, line, re.UNICODE)[0]
            #print(matches)
            id, lat, long, alt, types, name, pilot = matches
            if not bool([ele for ele in ["Kaplan 1-1", "nima3333", "Nouveau Surnom"] if(ele in pilot)]):
                continue
            new_obj = Object(name, int(id, 16), types, pilot, last_time, Coord(long, lat, alt))
            objects[int(id, 16)] = new_obj
        elif len(line) and line[0]== "-" and re.match(destruction_pattern, line):
            matches = re.findall(destruction_pattern, line, re.UNICODE)[0]
            id = int(matches, 16)
            obj = objects.get(id, None)
            if obj is None:
                continue
            obj.alive = False
            obj.destroy_time = last_time
    return objects, last_time

def proceed_tac(tacview_path):
    #print(tacview_path, tacview_path.suffixes)
    text = extract_zip(tacview_path)
    if text is None:
        return None

    objects, last_time = parse_tacview(text)

    d = defaultdict(lambda: 0)

    for e, i in objects.items():
        if bool([ele for ele in ["Kaplan 1-1", "nima3333", "Nouveau Surnom"] if(ele in i.pilot)]):
            if i.destroy_time is None:
                d[i.name] += (last_time - i.creation)/3600
            else:
                d[i.name] += (i.destroy_time - i.creation)/3600
    return Counter(d)

if __name__ == "__main__":
    argParser = argparse.ArgumentParser()
    argParser.add_argument("-p", "--path", help="path to tacview files", default="./")

    args = argParser.parse_args()
    tacview_paths = Path(args.path).glob("*.zip.acmi")
    tacview_paths = list(tacview_paths)

    res = Counter()
    with Pool() as pool:
        for result in pool.imap_unordered(proceed_tac, tqdm(tacview_paths)):
            if result is not None:
                res = res + result
    print(res)
