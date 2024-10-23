import polars as pl
import polars.selectors as cs
import time
from memory_profiler import memory_usage

def read_dataset(file_path):
    df = None
    if file_path.endswith(".csv"):
        df = pl.read_csv(file_path)
    elif file_path.endswith(".xlsx"):
        df = pl.read_excel(file_path)  # Polars supports Excel reading
    return df

def generate_summary_statistics(df):
    # Using polars to generate summary statistics
    summary = df.select(pl.all()).describe()
    df = df.select(cs.by_dtype(pl.NUMERIC_DTYPES))
    mean_values = df.select(pl.all().mean())
    median_values = df.select(pl.all().median())
    print(median_values)
    std_dev = df.select(pl.all().std())
    return summary, mean_values, median_values, std_dev

def measure_time_and_memory(df):
    mem_before = memory_usage()[0]
    start_time = time.time()

    generate_summary_statistics(df)

    end_time = time.time()
    mem_after = memory_usage()[0]
        
    # Calculate elapsed time and memory used
    elapsed_time = (end_time - start_time)* 1000
    memory_used = mem_after - mem_before
    
    # Return the result along with time and memory usage
    print(f"Elapsed time: {elapsed_time:.2f} ms")
    print(f"Memory used: {memory_used * 1024:.2f} KB")
    return elapsed_time, memory_used * 1024 

# def create_save_visualization(df, column_name, save_filename=None, show=False):
#     sns.set_theme(style="whitegrid")
#     plt.figure(figsize=(8, 6))
#     # Convert to numpy for visualization in seaborn
#     sns.histplot(df[column_name].to_numpy(), kde=True, color="skyblue", bins=30)
#     plt.title(f"{column_name} Distribution", fontsize=16)
#     plt.xlabel(column_name, fontsize=12)
#     plt.ylabel("Frequency", fontsize=12)

#     if save_filename:
#         plt.savefig(save_filename, bbox_inches="tight")
#     if show:
#         plt.show()


# def generate_report(df, title):
#     # Generate summary statistics using polars
#     summary_stats, mean_values, median_values, std_dev = generate_summary_statistics(df)

#     with open(title + ".md", "w", encoding="utf-8") as file:
#         file.write("# Summary Report\n\n")
#         file.write("| Metric          | Value           |\n")
#         for i in range(len(summary_stats.columns)):
#             file.write("|------------------|----------------|\n")
#             for column in summary_stats.columns:
#                 value = summary_stats[column][
#                     i
#                 ]  # Get the first (and only) value from the Series
#                 file.write(f"| **{column}**      | {value}        |\n")
#         file.write("\n")

#         file.write("## Mean Values:\n")
#         for column in mean_values.columns:
#             mean = mean_values[column][
#                 0
#             ]  # Get the first (and only) value from the Series
#             file.write(f"- **{column}**: {mean}\n")
#         file.write("\n")

#         file.write("## Median Values:\n")
#         for column in median_values.columns:
#             median = median_values[column][
#                 0
#             ]  # Get the first (and only) value from the Series
#             file.write(f"- **{column}**: {median}\n")
#         file.write("\n")

#         file.write("## Standard Deviation:\n")
#         for column in std_dev.columns:
#             std = std_dev[column][0]  # Get the first (and only) value from the Series
#             file.write(f"- **{column}**: {std}\n")
#         file.write("\n")

#         file.write("## Distributions:\n")
#         file.write("![Age Distribution](Age_distribution.png)\n\n")
#         file.write("![Fare Distribution](Fare_distribution.png)\n\n")
#         file.write("![Pclass Distribution](Pclass_distribution.png)\n")
# file_path = (
#     "https://web.stanford.edu/class/archive/cs/cs109/cs109.1166/stuff/titanic.csv"
# )
# df = read_dataset(file_path)
# elapsed_time, memory_used = measure_time_and_memory(df)
