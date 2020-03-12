import sys

import csv
import json

def csv_to_json():
    csv_filename = sys.argv[1]
    json_filename = sys.argv[2]

    with open(csv_filename) as csv_file:
        reader = csv.DictReader(csv_file)
        csv_rows = list(reader)
        with open(json_filename, 'w+') as json_file:
            json.dump(csv_rows, json_file)

def json_to_csv():
    json_filename = sys.argv[1]
    csv_filename = sys.argv[2]

    with open(json_filename) as json_file:
        with open(csv_filename, 'w+') as csv_file:
            json_data = json.load(json_file)
            csv_writer = csv.writer(csv_file)

            csv_writer.writerow(json_data[0].keys())

            for row in json_data:
                csv_writer.writerow(row.values())
