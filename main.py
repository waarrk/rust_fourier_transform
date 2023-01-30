import time
import subprocess
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np


def transform(input_file_name, output_file_name):
    start = time.perf_counter()
    with open(output_file_name, 'w') as f:
        subprocess.run(['./target/debug/rust_fourier_transform',
                       input_file_name, '8000'], stdout=f, stderr=subprocess.DEVNULL)
    return (time.perf_counter() - start)


def graph(input_wave, output_wave, max_freq):
    plt.figure("Rust Fourier Spectrum Display")

    plt.subplot(2, 1, 1)
    plt.title("frequency vs gain on case 2")
    plt.plot(input_wave["time"] * 1000,
             input_wave["amp"], lw=0.5, color="blue")
    plt.xlim(left=0, right=(1/max_freq) * 1000)

    plt.xlabel("time [msec]")
    plt.ylabel("amp [dB]")
    plt.grid()

    plt.subplot(2, 1, 2)
    plt.plot(output_wave["freq"], output_wave["amp"], lw=0.5, color="red")

    plt.xlabel("frequency [Hz]")
    plt.ylabel("gain [dB]")

    plt.ylim(bottom=0)
    plt.grid()
    plt.show()


if __name__ == '__main__':
    input_file_name = "sin_100_500_2000.txt"  # 入力ファイル名
    output_file_name = "out.csv"  # 出力ファイル名
    max_freq = 200  # 予想される最大周波数

    print("-- 関数読み込み --")
    input_wave = pd.read_csv(input_file_name, delimiter=",",
                             header=None, names=("time", "amp"))

    print("-- 解析開始 --")
    elapsed_time = transform(input_file_name, output_file_name)

    print("-- 解析終了 -- time: {:.10f} sec".format(elapsed_time))

    output_wave = pd.read_csv(output_file_name, delimiter=",",
                              header=None, names=("freq", "amp"))

    print("-- グラフ描画 --")
    # matplotlib
    graph(input_wave, output_wave, max_freq)
