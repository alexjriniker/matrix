import csv
import numpy as np
import matplotlib.pyplot as plt


# From stackoverflow https://stackoverflow.com/questions/11686720/is-there-a-numpy-builtin-to-reject-outliers-from-a-list
def reject_outliers(data, m=2.0):
    d = np.abs(data - np.median(data))
    mdev = np.median(d)
    s = d / mdev if mdev else np.zeros(len(d))
    return data[s < m]


def main():
    plt.title("Histogram of (|predicted - actual| / max_off)")

    # plt.xscale('symlog')
    plt.xlabel("Percentage off (%)")

    # plt.yscale("log")
    plt.ylabel("Number of Times (#)")

    with open("out.csv", "r") as f:
        for data in csv.reader(f):
            y = reject_outliers(np.array(list(map(float, data))), 20)
            counts, bins = np.histogram(y, bins=70)

            plt.hist(bins[:-1], bins, weights=counts)
            print(f"{counts=} {bins=}")

    plt.show()


if __name__ == "__main__":
    main()
