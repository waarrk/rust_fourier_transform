import csv


def convert_to_csv(input_file, output_file):
    data = []
    sample_rate = 44100
    now = 0
    with open(input_file) as f:
        for line in f:
            row = line.strip().split("\t")
            data.append([float(now), float(row[0])])
            now += 1/sample_rate

    with open(output_file, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerows(data)


if __name__ == "__main__":
    input_file = "sample-data2.txt"
    output_file = "sample_2.csv"
    convert_to_csv(input_file, output_file)
