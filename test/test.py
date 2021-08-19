import os
from rich.console import Console
from rich.table import Table
import subprocess
import re
from pprint import pprint
import csv


def main():

    # testrange = [
    # 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    # 23, 24, 25, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 42, 43, 44, 45, 46,
    # 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 60, 61, 62, 63, 64, 65, 66, 67,
    # 75, 76, 77, 78, 79, 80, 81,
    # 82,  # rational numbers
    # 83, 84,
    # 85,  # Enotation
    # 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
    # 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
    # 121, 122, 123, 124, 125, 126, 127, 128, 132, 133, 135, 136, 137, 138, 139, 140,
    # 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
    # 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 173,
    # 174, 175, 176, 177, 178, 179, 180, 181, 183,
    # 185,  # Enotation
    # 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197,
    # 198,  # Math AND in piecewise func
    # 199,
    # 200,  # Math OR
    # 201,  # MATH XOR
    # 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217,
    # 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233,
    # 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    # 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265,
    # 266, 267, 268, 269, 270,
    # 271,  # Ci in lambda function
    # 272, 273, 274,
    # 275,  # Ci in lambda func
    # 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291,
    # 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307,
    # 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323,
    # 324, 325, 326, 327,
    # 328,
    # 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339,
    # 340, 341, 342, 343, 344, 345, 346, 347,
    # 462,
    # 463, 464, 465, 466, 467, 468, 469,
    # 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485,
    # 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501,
    # 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 522, 523,
    # 524, 525, 526, 527, 528, 529, 530, 577, 578, 579, 580, 581, 582, 583, 584, 585,
    # 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 598, 599, 600, 601, 602,
    # 603, 604, 605, 606, 607, 608, 611, 612, 616, 617, 618, 625, 626, 627, 631, 632,
    # 633, 640, 641, 642, 643, 644, 645, 667, 668, 669, 670, 671, 672, 676, 677, 678,
    # 685, 686, 688, 691, 692, 693, 694, 697, 698, 703, 704, 706, 709, 710, 711, 712,
    # 713, 714, 715, 716, 717, 718, 719, 720, 721, 722, 732, 733, 734, 735, 738, 739,
    # 740, 741, 742, 781, 782, 783, 784, 785, 786, 787, 788, 792, 793, 794, 795, 796,
    # 797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812,
    # 813, 814, 815, 816, 817, 818, 819, 820, 821, 822, 823, 824, 825, 826, 830, 831,
    # 832, 833, 834, 835, 836, 837, 838, 839, 840, 841, 842, 843, 901, 902, 903, 904,
    # 905, 906, 907, 908, 909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920,
    # 921, 922, 923, 924, 925, 926, 927, 949, 950, 951, 954, 956, 957, 958, 969, 970,
    # 971, 974, 998, 999, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010,
    # 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1019, 1020, 1021, 1022, 1023,
    # 1024, 1025, 1026, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039,
    # 1040, 1041, 1042, 1043, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063,
    # 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1077, 1078, 1079, 1080, 1081, 1082,
    # 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1096, 1097, 1098, 1102, 1103, 1104,
    # 1105, 1107, 1110, 1111, 1112, 1113, 1114, 1115, 1116, 1117, 1118, 1122, 1123,
    # 1184, 1185, 1197, 1198, 1199, 1200, 1201, 1202, 1203, 1204, 1205, 1206, 1207,
    # 1208, 1209, 1210, 1215, 1216, 1217, 1218, 1219, 1220, 1221, 1224, 1225, 1226,
    # 1231, 1232, 1233, 1234, 1235, 1236, 1245, 1246, 1247, 1271, 1272, 1273, 1274,
    # 1275, 1276, 1300, 1301, 1302, 1307, 1308, 1309, 1310, 1311, 1312, 1313, 1314,
    # 1315, 1338, 1339, 1341, 1342, 1395, 1420, 1421, 1422, 1423, 1424, 1425, 1426,
    # 1427, 1428, 1429, 1430, 1431, 1432, 1433, 1434, 1436, 1438, 1440, 1442, 1449,
    # 1450, 1451, 1452, 1453, 1464, 1465, 1478, 1485, 1486, 1489, 1490, 1491, 1492,
    # 1493, 1494, 1498, 1513, 1514, 1515, 1516, 1552, 1553, 1554, 1555, 1556, 1557,
    # 1561, 1563, 1564, 1566, 1574, 1631, 1633, 1635, 1654, 1655, 1657, 1742, 1746,
    # 1760, 1761, 1766, 1767, 1768, 1773, 1774
    # ]

    skipModels = []
    skipTags = ['EventNoDelay', 'EventWithDelay', 'ConversionFactors',
                'AlgebraicRule', 'CSymbolTime', 'CSymbolDelay', 'CSymbolAvogadro', 'CSymbolRateOf']
    tagMetrics = {
        "componentTags": {},
        "testTags": {}
    }

    console = Console()
    passed = 0
    failed = 0
    total = 1780
    for directory in range(1, total + 1):
        if directory in skipModels:
            console.print(f"[yellow]{directory:05} SKIPPED")
            continue
        common_filename = '../testsuites/core-semantic/' + \
            f'{directory:05}' + '/' + f'{directory:05}'
        errors_found = False

        settings_filename = common_filename + '-settings.txt'
        settings = read_settings(settings_filename)
        console.print(f"[yellow]{directory:05} ", end="")

        os.chdir('../sbml-sim')
        model_filename = common_filename + '-sbml-l3v2.xml'
        description_filename = common_filename + '-model.m'
        standard_results_filename = common_filename + '-results.csv'
        description = get_model_description(description_filename)
        if '3.2' not in description["levels"]:
            console.print(
                f"[blue]SKIPPED because v3.2 model not present")
            continue
        if description['packagesPresent'] != []:
            console.print(
                f"[blue]SKIPPED because packages present: {description['packagesPresent']}")
            continue
        skip = False
        for skipTag in skipTags:
            if skipTag in description["componentTags"] + description["testTags"]:
                console.print(
                    f"[blue]SKIPPED because {skipTag} not implemented")
                skip = True
                break
        if skip:
            continue

        simulator_results = get_simulator_results(model_filename, settings)
        if simulator_results == {}:
            errors_found = True
        else:
            # console.print()
            # try:
            os.chdir('../rust-sbml-test')
            libsbmlsim_results = get_libsbmlsim_results(
                model_filename, settings)
            standard_results = get_standard_results(
                standard_results_filename)
            if libsbmlsim_results != {}:
                # print("sanity check")
                sanity_check = compare_results(libsbmlsim_results, standard_results, float(
                    settings['relative']), float(settings['absolute']))
                if sanity_check != []:
                    console.print(
                        f"[blue]libSBMLsim doesn't pass this test ", end="")
                # else:
                    # console.print(f"[blue]SANITY CHECK PASSED ", end="")
                # print("Comparing with libsbml")
                libsbmlsim_errors = compare_results(
                    simulator_results, libsbmlsim_results, float(settings['relative']), float(settings['absolute']))

                if libsbmlsim_errors != []:
                    errors_found = True
                    # print("\nlibsbmlsim_errors")

            # print("\nComparing with standard")
            standard_errors = compare_results(
                simulator_results, standard_results, float(settings['relative']), float(settings['absolute']))
            if standard_errors != []:
                errors_found = True
                # print("standard_errors")

        if not errors_found:
            console.print("[green]PASS", end="")
            passed += 1
        else:
            console.print("[red]FAIL", end="")
            print()
            print(description)
            failed += 1
            # print("SETTINGS")
            # pprint(settings)
            # print("SIMULATOR RESULTS")
            # pprint(simulator_results)
            # print("STANDARD RESULTS")
            # pprint(standard_results)
            # exit()
        tagMetrics = refresh_tag_metrics(
            tagMetrics, description, not errors_found)
        percent = int(passed/(passed+failed)*100)
        print(f"  {passed=}, {failed=}, {percent=}%")
    printMetrics(tagMetrics)


def read_settings(filename):

    settings = {}

    with open(filename, 'r') as file:
        for line in file.readlines():
            line = line.strip()
            if line == '':
                continue
            key, value = tuple(line.split(':'))
            value = value.strip()
            if value == '':
                continue
            if key in ['variables', 'amount']:
                value = [x.strip() for x in value.split(',')]
            elif key in ['relative', 'absolute']:
                value = float(value)
            elif ',' in value:
                value = value.split(',')
                value = list(map(lambda x: x.strip(), value))
            settings[key] = value

    return settings


def get_simulator_results(model_filename, settings):

    rtol = 1e-10
    atol = 1e-16
    method = "RKF45"
    command = ['./target/release/sbml-sim',
               f'{model_filename}',
               settings['duration'],
               settings['steps'], method, str(rtol), str(atol)]
    if 'amount' in settings.keys():
        amount_flag = '-a'
        command += [amount_flag]

    # print(command)
    cargo_result = subprocess.Popen(command, stdout=subprocess.PIPE,
                                    stderr=subprocess.STDOUT)
    stdout, stderr = cargo_result.communicate()
    output = str(stdout, encoding="UTF-8")

    if 'panicked' in output:
        return {}

    output = output.split('\n')[2:]

    headings = output[0].split()

    output = [x.split() for x in output[1:-1]]

    if len(headings) < len(output[0]):
        return {}

    # print(output)
    result = {}
    for row in output:
        t = row[0]
        result[t] = {}

        col = 1
        while col < len(output[0]):
            species = headings[col]
            amount = row[col]
            result[t][species] = float(amount)
            col += 1

    return result


def get_libsbmlsim_results(model_filename, settings):
    command = ['simulateSBML', '-t', str(float(settings['duration'])),
               '-s', str(int(float(settings['steps']))),
               '-m', '13',
               '-A', '1e-16',
               '-R', '1e-10',
               f'{model_filename}']

    if 'amount' in settings.keys():
        amount_flag = '-a'
        command += [amount_flag]

    # print(command)
    libsbmlsim_results = subprocess.Popen(command,
                                          stdout=subprocess.PIPE,
                                          stderr=subprocess.STDOUT)

    stdout, stderr = libsbmlsim_results.communicate()
    output = str(stdout, encoding="UTF-8")
    # print(output.strip())

    result = {}
    try:
        with open('out.csv', 'r') as file:
            reader = csv.DictReader(file)
            species = list(filter(lambda x: 'S' in x, reader.fieldnames))
            for row in reader:
                t = format(float(row['time']), '.10f')
                result[t] = {}
                for sp in species:
                    result[t][sp] = float(row[sp])
    except:
        return result
    # print(result)
    os.remove('out.csv')

    return result


def get_standard_results(standard_results_filename):
    result = {}
    with open(standard_results_filename, 'r') as file:
        reader = csv.DictReader(file)
        species = list(filter(lambda x: 'S' in x, reader.fieldnames))
        for row in reader:
            if 'time' in row.keys():
                t = format(float(row['time']), '.10f')
            else:
                t = format(float(row['Time']), '.10f')
            result[t] = {}
            for sp in species:
                result[t][sp] = float(row[sp])
    return result


def compare_results(simulator_results, standard_results, relative_error, absolute_error):
    errors = []
    # if len(simulator_results) != len(standard_results):
    # print(simulator_results)
    # print(standard_results)
    # exit()
    # print(simulator_results, standard_results)

    time_points = list(map(lambda x: format(x, '.10f'), sorted(list(
        map(lambda x: float(x), simulator_results.keys())))))
    for t in time_points:
        if t not in simulator_results.keys():
            errors.append(f"{t} not in simulator results")
            continue
            # print(f"{t} not in simulator results")
            # print(time_points)
            # print(sorted(simulator_results.keys()))
            # print(simulator_results)
        if t not in standard_results.keys():
            errors.append(f"{t} not in standard_results results")
            continue
            # print(time_points)
            # print(sorted(standard_results.keys()))
            # print(standard_results)
        simulator_result = simulator_results[t]
        standard_result = standard_results[t]
        error = compare_at_time(t,
                                simulator_result, standard_result, relative_error, absolute_error)
        if error is not None:
            errors.append(error)
            # print(f"error at t = {t}")
            # print(errors)
            return errors
    return errors


def compare_at_time(t, sim, exp, relative_error, absolute_error):
    errors = {
        'keys': [],
        'diffs': [],
        'max_diff': 0
    }
    error_count = 0

    for key in exp.keys():
        if key not in sim.keys():
            print(f"{exp=}")
            print(f"{sim=}")
        diff = abs(exp[key] - sim[key])
        tol = exp[key] * relative_error + absolute_error
        if(diff <= tol):
            continue
        else:
            errors['diffs'].append(diff)
            errors['keys'].append(key)
            if diff > errors['max_diff']:
                errors['max_diff'] = diff
            error_count += 1
            # print(f"{t=}")
            # print(f"{key=}")
            # print(f"{exp[key]=}")
            # print(f"{sim[key]=}")
            # print(f"{diff=}")
            # print(f"{tol=}")
            # exit()

    if error_count > 0:
        return errors


def get_model_description(description_filename):

    description = {
        "componentTags": [],
        "testTags": [],
        "levels": [],
        "packagesPresent": []
    }

    with open(description_filename, 'r') as file:
        for line in file.readlines():
            for key in description.keys():
                if key + ":" in line:
                    line = line.split(':')[1]
                    items = line.strip().split(',')
                    description[key] = [x.strip()
                                        for x in items]
                    if '' in description[key]:
                        description[key].remove('')

    return description


def refresh_tag_metrics(metrics, current_model_desc, current_result):

    for tags in ["componentTags", "testTags"]:
        for tag in current_model_desc[tags]:
            if tag not in metrics[tags].keys():
                metrics[tags][tag] = {
                    "pass": 0, "fail": 0
                }

            if current_result:
                metrics[tags][tag]["pass"] += 1
            else:
                metrics[tags][tag]["fail"] += 1

    return metrics


def printMetrics(metrics):
    console = Console()
    for tags in ["componentTags", "testTags"]:
        console.print(f"[blue]{tags}")
        tags_results = []
        tags_results_pass_percentage = []
        for tag, results in metrics[tags].items():
            pass_percent = results["pass"] / \
                (results["pass"] + results["fail"]) * 100
            tags_results.append((tag, results, pass_percent))
            tags_results_pass_percentage.append(pass_percent)
        tags_results.sort(key=lambda x: x[2])
        for tag, results, pass_percent in tags_results:
            console.print(
                f"{tag:30}[green]{results['pass']:4}  [red]{results['fail']:4}  [yellow]{pass_percent:8}")


if __name__ == "__main__":
    main()
