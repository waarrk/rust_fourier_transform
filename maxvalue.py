import csv

def find_max_value_column1(file_path):
    max_value = 0
    column1_value = 0
    with open(file_path, "r") as csvfile:
        reader = csv.reader(csvfile)
        for row in reader:
            column2_value = float(row[1])
            if column2_value > max_value:
                max_value = column2_value
                column1_value = float(row[0])
    return column1_value

if __name__ == '__main__':
  file_path = "path.csv"
  print(find_max_value_column1(file_path))