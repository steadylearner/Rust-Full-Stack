import webbrowser as web

# import csv

def read_post_log():
    prefix = "https://github.com/steadylearner/Python-Blog/blob/master"
    filename = "post_log.csv"

    web.open(f"{prefix}/{filename}")

    # with open("post_log.csv") as csv_file:
    #     csv_reader = csv.reader(csv_file, delimiter=',')
    #     line_count = 0
    #     for row in csv_reader:
    #         if line_count == 0:
    #             print(f'\n\tColumn {", ".join(row)}')
    #             line_count += 1
    #         else:
    #             print(f'\n\t{row[2]} - {row[1]}')
    #             line_count += 1
    #     print(f'\n\t Processed {line_count} lines.')