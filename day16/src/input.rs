pub fn init() -> (Vec<[u64;20]>,[u64;20]) {
    let nearby_ticket = vec![
        [
            946, 293, 123, 156, 812, 202, 75, 884, 162, 420, 436, 60, 720, 719, 272, 642, 515, 724,
            150, 875,
        ],
        [
            221, 912, 887, 284, 758, 512, 907, 690, 384, 802, 742, 118, 318, 330, 282, 232, 507,
            470, 871, 615,
        ],
        [
            810, 431, 649, 91, 53, 784, 300, 722, 904, 166, 210, 75, 491, 87, 892, 625, 211, 151,
            434, 608,
        ],
        [
            643, 779, 232, 722, 326, 212, 626, 867, 265, 308, 561, 565, 322, 59, 466, 126, 60, 398,
            462, 902,
        ],
        [
            96, 926, 879, 397, 752, 863, 721, 283, 648, 731, 790, 314, 787, 127, 646, 123, 132,
            228, 932, 611,
        ],
        [
            170, 762, 169, 400, 589, 606, 232, 783, 97, 404, 97, 187, 404, 398, 436, 316, 736, 50,
            149, 637,
        ],
        [
            219, 131, 774, 321, 403, 362, 424, 109, 434, 387, 434, 650, 862, 847, 229, 645, 214,
            941, 738, 75,
        ],
        [
            781, 825, 387, 409, 418, 393, 392, 230, 501, 722, 819, 877, 716, 596, 636, 114, 82, 75,
            457, 921,
        ],
        [
            780, 114, 230, 227, 784, 267, 210, 318, 78, 262, 258, 902, 91, 464, 728, 313, 986, 753,
            434, 847,
        ],
        [
            721, 89, 151, 101, 423, 52, 305, 761, 679, 920, 683, 835, 887, 248, 643, 842, 87, 422,
            409, 264,
        ],
        [
            276, 288, 942, 66, 888, 195, 83, 149, 840, 322, 844, 323, 112, 866, 167, 276, 630, 813,
            850, 21,
        ],
        [
            156, 598, 212, 318, 509, 193, 116, 550, 560, 863, 716, 587, 906, 461, 229, 442, 980,
            254, 103, 276,
        ],
        [
            108, 878, 230, 391, 459, 715, 909, 358, 56, 136, 641, 921, 850, 107, 197, 446, 470,
            154, 257, 300,
        ],
        [
            429, 289, 136, 558, 869, 233, 649, 267, 725, 454, 781, 589, 343, 691, 528, 131, 821,
            784, 597, 919,
        ],
        [
            896, 385, 874, 717, 220, 630, 420, 643, 255, 613, 550, 817, 686, 218, 562, 347, 118,
            86, 470, 217,
        ],
        [
            382, 883, 106, 760, 432, 933, 925, 559, 780, 680, 944, 913, 465, 148, 56, 193, 401,
            300, 623, 459,
        ],
        [
            756, 615, 507, 213, 633, 332, 459, 606, 610, 426, 793, 547, 288, 520, 843, 439, 621,
            298, 382, 866,
        ],
        [
            628, 427, 555, 988, 766, 313, 875, 846, 924, 474, 877, 58, 319, 154, 606, 88, 311, 622,
            475, 625,
        ],
        [
            53, 884, 689, 473, 272, 87, 19, 466, 437, 429, 104, 470, 425, 107, 458, 411, 812, 294,
            159, 464,
        ],
        [
            228, 398, 462, 889, 629, 683, 894, 756, 724, 875, 107, 793, 129, 832, 781, 250, 319,
            166, 941, 51,
        ],
        [
            630, 811, 639, 755, 302, 235, 641, 869, 692, 130, 433, 859, 111, 258, 269, 632, 337,
            132, 114, 867,
        ],
        [
            262, 472, 683, 210, 308, 679, 101, 255, 60, 469, 684, 901, 212, 914, 418, 464, 793,
            947, 84, 130,
        ],
        [
            254, 905, 261, 864, 614, 866, 559, 879, 78, 466, 163, 632, 909, 516, 304, 396, 220,
            204, 875, 740,
        ],
        [
            826, 866, 633, 763, 840, 903, 772, 752, 558, 941, 916, 108, 102, 380, 754, 467, 357,
            313, 152, 452,
        ],
        [
            480, 635, 828, 720, 873, 145, 393, 473, 112, 734, 105, 344, 447, 334, 743, 678, 607,
            308, 90, 228,
        ],
        [
            466, 232, 907, 740, 752, 332, 219, 553, 388, 521, 635, 599, 549, 214, 330, 631, 288,
            129, 726, 599,
        ],
        [
            127, 398, 601, 902, 848, 297, 428, 691, 163, 58, 194, 94, 871, 152, 91, 881, 640, 516,
            808, 743,
        ],
        [
            416, 383, 432, 270, 357, 382, 624, 425, 105, 635, 734, 999, 759, 398, 61, 634, 635,
            471, 298, 58,
        ],
        [
            879, 762, 84, 305, 97, 912, 895, 149, 55, 105, 441, 167, 715, 57, 314, 840, 727, 331,
            174, 147,
        ],
        [
            404, 839, 97, 831, 462, 556, 562, 491, 925, 403, 558, 110, 793, 730, 131, 258, 599,
            271, 613, 688,
        ],
        [
            257, 417, 266, 66, 277, 743, 351, 811, 409, 760, 78, 254, 320, 465, 467, 457, 336, 945,
            217, 807,
        ],
        [
            82, 447, 444, 218, 915, 101, 599, 906, 394, 385, 210, 552, 472, 297, 562, 597, 411,
            194, 142, 359,
        ],
        [
            691, 130, 312, 559, 198, 290, 555, 484, 324, 151, 879, 221, 866, 402, 560, 165, 510,
            788, 285, 294,
        ],
        [
            520, 354, 609, 603, 826, 728, 263, 288, 478, 719, 164, 763, 435, 892, 905, 412, 286,
            126, 350, 690,
        ],
        [
            216, 637, 56, 842, 466, 925, 411, 828, 306, 833, 475, 269, 84, 270, 116, 892, 778, 270,
            399, 422,
        ],
        [
            887, 989, 786, 232, 438, 322, 629, 319, 422, 194, 302, 387, 57, 85, 267, 894, 111, 128,
            265, 801,
        ],
        [
            787, 150, 367, 820, 142, 79, 391, 340, 285, 883, 785, 616, 213, 76, 50, 398, 300, 724,
            233, 729,
        ],
        [
            349, 871, 621, 791, 787, 407, 635, 345, 890, 150, 271, 998, 733, 385, 893, 759, 302,
            273, 616, 341,
        ],
        [
            838, 741, 337, 254, 676, 405, 387, 120, 800, 442, 148, 329, 141, 106, 410, 143, 891,
            488, 449, 883,
        ],
        [
            221, 621, 686, 789, 612, 258, 631, 449, 104, 847, 100, 266, 274, 991, 817, 400, 271,
            98, 286, 889,
        ],
        [
            148, 276, 228, 770, 929, 797, 631, 412, 104, 759, 877, 401, 649, 767, 553, 213, 614,
            402, 310, 737,
        ],
        [
            310, 303, 587, 584, 466, 333, 589, 912, 763, 828, 947, 339, 108, 161, 618, 605, 354,
            235, 114, 941,
        ],
        [
            218, 223, 646, 96, 166, 108, 366, 883, 315, 154, 159, 650, 352, 602, 77, 252, 742, 515,
            404, 728,
        ],
        [
            391, 562, 716, 108, 919, 521, 511, 790, 430, 60, 678, 675, 197, 677, 221, 729, 295,
            340, 752, 305,
        ],
        [
            762, 916, 949, 888, 596, 516, 920, 868, 522, 464, 917, 512, 760, 274, 337, 874, 397,
            65, 880, 82,
        ],
        [
            520, 622, 286, 108, 471, 730, 811, 906, 553, 704, 465, 122, 302, 877, 151, 754, 436,
            877, 352, 415,
        ],
        [
            327, 163, 814, 291, 58, 731, 260, 393, 633, 555, 789, 941, 250, 166, 610, 776, 846, 95,
            575, 683,
        ],
        [
            726, 258, 342, 921, 146, 145, 914, 723, 349, 514, 838, 594, 632, 776, 925, 263, 850,
            930, 887, 336,
        ],
        [
            261, 692, 739, 924, 754, 138, 164, 826, 754, 551, 473, 871, 305, 131, 81, 759, 295,
            946, 862, 217,
        ],
        [
            753, 863, 309, 437, 285, 162, 867, 826, 704, 682, 753, 759, 684, 637, 801, 288, 301,
            734, 760, 259,
        ],
        [
            291, 686, 406, 163, 424, 473, 214, 762, 405, 227, 816, 556, 779, 324, 225, 844, 596,
            705, 644, 160,
        ],
        [
            285, 95, 276, 757, 394, 212, 982, 118, 290, 611, 843, 438, 611, 107, 438, 323, 843,
            877, 720, 152,
        ],
        [
            107, 131, 327, 719, 872, 345, 602, 423, 453, 84, 727, 733, 812, 691, 910, 327, 975,
            459, 261, 91,
        ],
        [
            728, 319, 568, 304, 878, 424, 125, 514, 848, 512, 74, 740, 785, 620, 215, 117, 419,
            839, 471, 315,
        ],
        [
            523, 817, 891, 422, 77, 232, 449, 557, 52, 437, 62, 801, 453, 297, 332, 100, 603, 948,
            848, 260,
        ],
        [
            356, 280, 894, 517, 219, 305, 281, 128, 55, 926, 116, 324, 822, 682, 354, 284, 810,
            528, 288, 232,
        ],
        [
            325, 465, 724, 401, 170, 690, 159, 589, 427, 376, 125, 228, 297, 847, 734, 771, 633,
            153, 717, 849,
        ],
        [
            445, 354, 404, 406, 724, 231, 414, 842, 297, 643, 147, 914, 918, 556, 214, 628, 55,
            764, 521, 518,
        ],
        [
            136, 111, 817, 913, 793, 892, 97, 881, 266, 916, 235, 684, 196, 335, 614, 620, 415,
            722, 757, 223,
        ],
        [
            280, 889, 234, 98, 166, 874, 813, 216, 584, 320, 612, 692, 818, 514, 397, 876, 91, 730,
            471, 928,
        ],
        [
            60, 820, 880, 562, 403, 444, 685, 692, 866, 233, 299, 210, 649, 763, 16, 92, 133, 783,
            601, 683,
        ],
        [
            930, 302, 453, 158, 382, 773, 639, 735, 356, 877, 791, 602, 257, 146, 723, 675, 640,
            198, 825, 396,
        ],
        [
            618, 80, 850, 401, 383, 560, 134, 821, 787, 92, 141, 313, 759, 216, 341, 400, 731, 649,
            770, 651,
        ],
        [
            921, 108, 916, 162, 288, 919, 591, 766, 911, 572, 587, 846, 474, 651, 225, 718, 527,
            443, 716, 249,
        ],
        [
            216, 259, 882, 317, 150, 83, 252, 144, 115, 509, 894, 228, 396, 285, 124, 744, 379,
            227, 903, 55,
        ],
        [
            317, 326, 381, 132, 63, 172, 850, 434, 938, 336, 475, 905, 848, 101, 906, 920, 232, 98,
            635, 265,
        ],
        [
            609, 819, 650, 731, 418, 387, 917, 647, 433, 102, 333, 678, 410, 89, 390, 781, 987,
            442, 59, 520,
        ],
        [
            395, 792, 904, 864, 789, 880, 401, 615, 148, 639, 515, 845, 442, 518, 909, 885, 922,
            511, 370, 83,
        ],
        [
            244, 916, 109, 728, 890, 455, 297, 593, 173, 841, 509, 520, 114, 101, 153, 99, 255,
            167, 441, 218,
        ],
        [
            99, 622, 881, 677, 398, 598, 441, 286, 779, 445, 459, 528, 453, 721, 887, 780, 54, 827,
            254, 17,
        ],
        [
            420, 349, 413, 216, 415, 898, 513, 716, 741, 122, 768, 927, 114, 282, 718, 285, 509,
            198, 431, 336,
        ],
        [
            814, 305, 302, 120, 302, 754, 233, 864, 298, 645, 119, 142, 282, 157, 492, 605, 198,
            605, 347, 205,
        ],
        [
            862, 948, 283, 145, 278, 324, 406, 872, 337, 245, 104, 766, 792, 58, 129, 424, 291,
            103, 328, 650,
        ],
        [
            847, 949, 875, 165, 824, 551, 440, 431, 7, 515, 113, 688, 869, 456, 268, 787, 475, 890,
            902, 125,
        ],
        [
            278, 389, 765, 399, 354, 724, 631, 833, 303, 326, 471, 106, 850, 928, 515, 327, 417,
            163, 811, 130,
        ],
        [
            131, 247, 870, 676, 823, 703, 395, 305, 157, 221, 115, 944, 790, 262, 559, 91, 225,
            767, 286, 738,
        ],
        [
            421, 618, 346, 141, 886, 860, 916, 113, 436, 731, 158, 915, 634, 823, 556, 233, 730,
            407, 551, 326,
        ],
        [
            172, 401, 850, 344, 922, 443, 457, 617, 417, 606, 297, 619, 829, 408, 880, 753, 780,
            159, 910, 288,
        ],
        [
            402, 875, 271, 149, 55, 558, 720, 729, 136, 880, 600, 692, 641, 863, 330, 907, 249,
            876, 822, 492,
        ],
        [
            282, 231, 454, 256, 293, 437, 273, 566, 195, 317, 133, 382, 817, 88, 826, 880, 520,
            850, 433, 460,
        ],
        [
            689, 94, 795, 752, 169, 519, 818, 248, 927, 820, 920, 400, 925, 150, 719, 145, 588,
            610, 388, 683,
        ],
        [
            790, 551, 431, 96, 271, 392, 915, 312, 175, 871, 393, 772, 827, 145, 347, 778, 400,
            121, 344, 635,
        ],
        [
            749, 337, 787, 153, 267, 212, 61, 214, 334, 450, 258, 65, 335, 917, 812, 517, 234, 109,
            882, 865,
        ],
        [
            395, 684, 597, 272, 640, 763, 333, 442, 901, 847, 101, 142, 717, 408, 281, 252, 87,
            775, 254, 824,
        ],
        [
            874, 787, 125, 63, 820, 768, 63, 443, 875, 235, 75, 995, 258, 339, 388, 171, 389, 850,
            220, 269,
        ],
        [
            306, 434, 927, 294, 265, 511, 841, 596, 446, 64, 344, 234, 295, 418, 690, 427, 259,
            993, 884, 357,
        ],
        [
            316, 225, 792, 249, 937, 735, 728, 89, 518, 343, 144, 424, 225, 921, 382, 222, 904,
            847, 83, 285,
        ],
        [
            417, 614, 514, 895, 569, 678, 210, 438, 388, 466, 867, 818, 825, 131, 454, 735, 271,
            60, 801, 348,
        ],
        [
            279, 294, 684, 768, 976, 801, 420, 722, 688, 258, 305, 80, 255, 923, 914, 725, 94, 322,
            642, 752,
        ],
        [
            429, 924, 232, 560, 451, 921, 90, 782, 773, 446, 516, 255, 942, 415, 759, 335, 392,
            784, 714, 517,
        ],
        [
            171, 744, 604, 431, 618, 389, 171, 396, 761, 866, 774, 816, 454, 448, 113, 372, 594,
            513, 142, 847,
        ],
        [
            690, 430, 145, 91, 612, 907, 145, 523, 819, 904, 414, 683, 341, 225, 389, 734, 145,
            270, 839, 888,
        ],
        [
            834, 317, 597, 429, 676, 247, 117, 812, 253, 421, 424, 125, 437, 605, 255, 144, 740,
            307, 281, 412,
        ],
        [
            536, 792, 911, 172, 941, 409, 737, 462, 291, 257, 443, 247, 511, 353, 257, 102, 334,
            609, 877, 461,
        ],
        [
            876, 323, 898, 553, 767, 394, 888, 924, 729, 396, 120, 399, 775, 716, 928, 77, 607,
            173, 317, 132,
        ],
        [
            20, 878, 561, 325, 613, 426, 415, 122, 300, 893, 914, 819, 508, 394, 722, 255, 145,
            528, 341, 401,
        ],
        [
            724, 945, 323, 775, 220, 452, 784, 808, 633, 78, 888, 350, 865, 317, 621, 50, 745, 947,
            439, 152,
        ],
        [
            503, 465, 912, 326, 948, 383, 881, 915, 649, 761, 635, 344, 811, 843, 862, 264, 516,
            784, 150, 689,
        ],
        [
            928, 904, 631, 285, 820, 726, 120, 874, 107, 75, 294, 389, 207, 111, 61, 754, 223, 109,
            417, 164,
        ],
        [
            422, 947, 95, 173, 758, 610, 344, 429, 554, 754, 108, 238, 646, 173, 285, 723, 352,
            520, 86, 634,
        ],
        [
            555, 232, 684, 615, 768, 562, 327, 253, 92, 827, 948, 524, 457, 227, 736, 947, 623,
            786, 60, 311,
        ],
        [
            676, 254, 358, 55, 318, 95, 757, 999, 82, 676, 384, 491, 876, 99, 682, 169, 426, 792,
            213, 424,
        ],
        [
            798, 214, 435, 882, 642, 509, 432, 826, 786, 684, 740, 84, 793, 762, 866, 389, 909,
            347, 752, 913,
        ],
        [
            365, 152, 65, 908, 257, 811, 775, 512, 229, 718, 468, 554, 949, 682, 647, 60, 452, 786,
            910, 432,
        ],
        [
            844, 157, 770, 917, 589, 88, 736, 513, 103, 101, 507, 792, 159, 686, 611, 468, 508,
            246, 402, 224,
        ],
        [
            619, 789, 839, 592, 427, 744, 60, 323, 50, 925, 235, 718, 577, 323, 761, 313, 162, 715,
            687, 825,
        ],
        [
            119, 445, 470, 307, 351, 880, 406, 933, 56, 895, 596, 411, 347, 722, 301, 210, 142,
            756, 155, 689,
        ],
        [
            844, 349, 853, 556, 605, 151, 645, 850, 611, 453, 394, 388, 154, 297, 772, 717, 90,
            163, 147, 197,
        ],
        [
            845, 610, 336, 324, 352, 889, 872, 337, 919, 893, 436, 234, 445, 569, 80, 919, 149,
            555, 639, 302,
        ],
        [
            146, 601, 789, 877, 867, 587, 759, 115, 906, 784, 647, 677, 57, 234, 393, 842, 894,
            825, 89, 485,
        ],
        [
            603, 274, 319, 401, 604, 928, 817, 845, 341, 883, 688, 499, 863, 875, 285, 106, 385,
            94, 741, 924,
        ],
        [
            688, 760, 426, 266, 290, 636, 319, 824, 949, 891, 871, 557, 145, 349, 421, 385, 774,
            370, 893, 871,
        ],
        [
            261, 528, 615, 370, 214, 507, 107, 350, 925, 336, 332, 618, 335, 424, 410, 213, 874,
            82, 215, 91,
        ],
        [
            119, 917, 551, 303, 562, 251, 301, 97, 112, 336, 916, 695, 641, 383, 256, 919, 730,
            403, 675, 732,
        ],
        [
            726, 317, 115, 269, 831, 904, 527, 110, 520, 251, 513, 847, 599, 519, 273, 287, 813,
            65, 389, 142,
        ],
        [
            550, 614, 257, 265, 624, 485, 93, 440, 722, 164, 757, 406, 127, 592, 154, 220, 864,
            840, 592, 435,
        ],
        [
            102, 74, 498, 86, 724, 274, 768, 646, 420, 684, 905, 173, 289, 52, 680, 472, 784, 920,
            784, 249,
        ],
        [
            550, 682, 420, 84, 446, 806, 159, 759, 91, 628, 731, 332, 388, 438, 155, 811, 256, 91,
            93, 221,
        ],
        [
            260, 90, 784, 224, 510, 401, 423, 141, 868, 869, 134, 558, 782, 319, 227, 902, 256,
            907, 356, 737,
        ],
        [
            228, 446, 471, 148, 155, 628, 627, 233, 104, 684, 818, 805, 387, 411, 171, 297, 781,
            409, 560, 761,
        ],
        [
            397, 599, 355, 639, 122, 349, 685, 82, 445, 262, 976, 272, 424, 146, 828, 211, 82, 729,
            283, 841,
        ],
        [
            784, 230, 268, 912, 704, 885, 168, 730, 338, 909, 389, 824, 757, 419, 194, 270, 610,
            647, 872, 638,
        ],
        [
            445, 778, 159, 419, 871, 63, 125, 814, 792, 160, 725, 840, 802, 948, 779, 262, 128,
            643, 395, 623,
        ],
        [
            855, 158, 288, 559, 685, 415, 839, 292, 905, 592, 89, 161, 211, 593, 195, 916, 922,
            456, 610, 330,
        ],
        [
            405, 408, 731, 731, 561, 282, 124, 147, 300, 148, 993, 410, 166, 945, 549, 418, 651,
            459, 435, 279,
        ],
        [
            608, 263, 740, 844, 484, 156, 117, 78, 622, 164, 680, 329, 433, 770, 355, 452, 84, 76,
            117, 621,
        ],
        [
            214, 273, 61, 457, 451, 356, 868, 766, 839, 469, 118, 299, 334, 423, 948, 790, 558,
            258, 85, 21,
        ],
        [
            328, 117, 384, 80, 50, 172, 730, 612, 459, 650, 327, 120, 868, 276, 461, 411, 687, 188,
            774, 587,
        ],
        [
            647, 138, 257, 745, 844, 213, 743, 876, 600, 226, 509, 731, 112, 249, 198, 515, 745,
            555, 633, 877,
        ],
        [
            340, 996, 905, 104, 736, 685, 295, 737, 291, 627, 614, 893, 102, 270, 676, 293, 404,
            520, 885, 744,
        ],
        [
            289, 426, 920, 599, 474, 460, 642, 768, 161, 326, 839, 286, 877, 203, 153, 643, 235,
            310, 343, 346,
        ],
        [
            683, 415, 553, 400, 460, 645, 650, 589, 763, 120, 715, 883, 607, 611, 78, 222, 834,
            266, 767, 87,
        ],
        [
            475, 195, 421, 820, 467, 303, 567, 468, 457, 214, 553, 86, 816, 218, 160, 603, 516,
            107, 349, 684,
        ],
        [
            387, 754, 155, 348, 510, 193, 757, 501, 297, 790, 908, 440, 234, 96, 115, 720, 113,
            612, 556, 321,
        ],
        [
            317, 216, 58, 724, 825, 597, 790, 305, 237, 148, 459, 285, 872, 763, 642, 77, 848, 893,
            647, 553,
        ],
        [
            52, 912, 118, 267, 299, 399, 736, 558, 292, 691, 780, 768, 81, 841, 717, 195, 776, 994,
            715, 311,
        ],
        [
            725, 146, 117, 600, 279, 413, 450, 92, 465, 94, 120, 317, 641, 878, 310, 416, 990, 82,
            267, 309,
        ],
        [
            231, 381, 649, 562, 452, 381, 869, 813, 75, 262, 910, 374, 321, 744, 631, 263, 820,
            266, 125, 251,
        ],
        [
            319, 765, 769, 294, 767, 147, 551, 850, 728, 929, 319, 51, 793, 284, 558, 946, 494,
            125, 885, 676,
        ],
        [
            651, 435, 893, 270, 769, 755, 691, 210, 824, 305, 593, 840, 643, 320, 226, 594, 923,
            684, 202, 337,
        ],
        [
            412, 106, 787, 54, 628, 330, 322, 551, 325, 883, 210, 519, 463, 443, 818, 827, 203,
            165, 775, 249,
        ],
        [
            615, 225, 420, 435, 589, 310, 156, 319, 361, 904, 782, 827, 352, 516, 781, 63, 730,
            355, 415, 335,
        ],
        [
            263, 770, 330, 716, 112, 412, 113, 336, 388, 922, 454, 465, 723, 449, 928, 295, 327,
            522, 451, 727,
        ],
        [
            743, 307, 677, 114, 888, 243, 247, 645, 820, 85, 341, 556, 321, 302, 821, 635, 340,
            762, 814, 516,
        ],
        [
            208, 288, 116, 558, 677, 737, 397, 613, 838, 148, 159, 903, 355, 557, 260, 408, 873,
            732, 826, 331,
        ],
        [
            310, 775, 201, 840, 351, 329, 215, 887, 828, 100, 742, 341, 527, 256, 274, 57, 587,
            528, 317, 131,
        ],
        [
            881, 424, 948, 449, 217, 358, 739, 601, 886, 431, 513, 269, 447, 914, 922, 335, 845,
            598, 881, 11,
        ],
        [
            755, 622, 937, 520, 771, 169, 868, 158, 620, 444, 165, 839, 775, 781, 315, 914, 597,
            615, 779, 879,
        ],
        [
            843, 407, 475, 217, 893, 107, 781, 892, 165, 596, 196, 895, 119, 787, 738, 411, 19,
            611, 790, 745,
        ],
        [
            5, 155, 314, 381, 107, 617, 904, 785, 300, 251, 842, 83, 322, 171, 51, 310, 310, 783,
            594, 211,
        ],
        [
            877, 384, 66, 513, 406, 645, 941, 893, 353, 412, 196, 924, 738, 135, 216, 631, 758,
            257, 716, 77,
        ],
        [
            120, 408, 477, 848, 343, 438, 306, 337, 406, 211, 63, 336, 871, 170, 304, 277, 917,
            685, 272, 129,
        ],
        [
            800, 427, 121, 123, 498, 57, 817, 651, 403, 646, 929, 868, 291, 394, 779, 944, 515,
            385, 719, 555,
        ],
        [
            387, 639, 844, 629, 876, 428, 906, 60, 149, 513, 116, 335, 867, 780, 247, 140, 309,
            292, 411, 114,
        ],
        [
            551, 877, 761, 764, 446, 254, 348, 320, 737, 873, 87, 758, 126, 168, 997, 357, 164,
            231, 329, 681,
        ],
        [
            443, 611, 169, 429, 745, 644, 821, 826, 224, 871, 821, 740, 975, 917, 145, 133, 911,
            902, 102, 210,
        ],
        [
            142, 296, 591, 273, 141, 691, 430, 150, 806, 247, 173, 116, 228, 434, 755, 78, 271, 51,
            507, 358,
        ],
        [
            105, 838, 744, 634, 279, 325, 259, 677, 160, 813, 682, 745, 632, 277, 438, 581, 949,
            826, 876, 349,
        ],
        [
            729, 528, 689, 459, 291, 413, 626, 253, 639, 843, 920, 320, 61, 460, 620, 429, 651,
            877, 829, 650,
        ],
        [
            738, 977, 507, 93, 51, 825, 638, 340, 791, 132, 92, 106, 164, 439, 736, 864, 787, 382,
            198, 492,
        ],
        [
            440, 739, 717, 453, 528, 844, 122, 731, 435, 59, 739, 911, 85, 557, 114, 690, 453, 443,
            514, 581,
        ],
        [
            224, 617, 74, 644, 559, 818, 884, 286, 641, 869, 815, 324, 259, 157, 588, 338, 145,
            139, 386, 758,
        ],
        [
            462, 915, 444, 387, 740, 222, 456, 515, 761, 332, 985, 884, 491, 315, 462, 401, 93,
            275, 814, 259,
        ],
        [
            346, 297, 277, 551, 865, 850, 596, 223, 515, 119, 149, 990, 283, 169, 527, 887, 343,
            887, 411, 445,
        ],
        [
            258, 161, 56, 909, 107, 684, 77, 117, 627, 846, 474, 550, 839, 770, 435, 916, 873, 942,
            899, 261,
        ],
        [
            728, 635, 474, 149, 846, 253, 514, 339, 839, 301, 394, 882, 153, 333, 19, 154, 328,
            732, 234, 620,
        ],
        [
            778, 308, 392, 162, 471, 436, 347, 786, 922, 332, 288, 930, 761, 815, 77, 551, 357,
            947, 129, 777,
        ],
        [
            588, 303, 84, 771, 592, 50, 55, 426, 436, 123, 549, 799, 610, 914, 608, 57, 916, 452,
            164, 406,
        ],
        [
            721, 334, 402, 395, 874, 406, 128, 55, 764, 734, 323, 519, 81, 81, 435, 170, 314, 802,
            785, 106,
        ],
        [
            733, 154, 184, 879, 391, 614, 349, 620, 783, 607, 342, 438, 235, 614, 304, 398, 527,
            59, 717, 914,
        ],
        [
            103, 467, 338, 305, 272, 162, 877, 276, 336, 340, 301, 631, 681, 945, 99, 419, 162,
            830, 302, 267,
        ],
        [
            327, 984, 848, 60, 61, 845, 948, 438, 843, 332, 214, 102, 780, 681, 353, 845, 849, 765,
            550, 401,
        ],
        [
            646, 609, 527, 595, 840, 592, 401, 637, 354, 926, 305, 428, 220, 887, 340, 761, 937,
            870, 905, 332,
        ],
        [
            419, 442, 997, 722, 293, 269, 928, 780, 59, 474, 51, 516, 844, 212, 459, 457, 744, 159,
            554, 512,
        ],
        [
            102, 761, 296, 396, 675, 403, 680, 277, 193, 938, 878, 735, 382, 789, 941, 431, 731,
            105, 104, 492,
        ],
        [
            914, 755, 911, 212, 90, 248, 635, 404, 650, 501, 718, 277, 731, 165, 266, 608, 90, 474,
            553, 692,
        ],
        [
            418, 295, 299, 307, 813, 500, 320, 411, 160, 492, 386, 445, 288, 90, 639, 83, 678, 440,
            880, 461,
        ],
        [
            170, 771, 115, 413, 446, 631, 343, 727, 720, 248, 598, 787, 482, 619, 596, 133, 912,
            882, 262, 846,
        ],
        [
            649, 352, 766, 643, 165, 928, 152, 50, 864, 210, 812, 748, 430, 399, 218, 691, 590,
            173, 774, 343,
        ],
        [
            419, 156, 318, 733, 63, 402, 292, 147, 731, 163, 471, 213, 219, 488, 512, 606, 917,
            132, 262, 436,
        ],
        [
            265, 465, 552, 945, 690, 758, 315, 131, 692, 866, 410, 271, 101, 949, 559, 881, 103,
            329, 12, 344,
        ],
        [
            684, 599, 816, 332, 298, 716, 439, 394, 908, 865, 919, 638, 155, 555, 99, 103, 474,
            813, 65, 135,
        ],
        [
            475, 761, 928, 316, 791, 895, 415, 916, 391, 59, 626, 341, 289, 82, 599, 981, 818, 432,
            441, 715,
        ],
        [
            170, 683, 840, 872, 207, 718, 161, 868, 924, 908, 616, 515, 255, 64, 866, 59, 463, 256,
            850, 511,
        ],
        [
            118, 461, 701, 231, 838, 605, 677, 817, 550, 871, 50, 792, 467, 634, 742, 725, 305,
            155, 400, 288,
        ],
        [
            685, 52, 346, 104, 649, 397, 886, 388, 79, 894, 414, 93, 134, 491, 161, 249, 342, 225,
            603, 949,
        ],
        [
            552, 398, 525, 778, 620, 872, 294, 552, 870, 325, 755, 403, 457, 463, 130, 226, 171,
            87, 321, 329,
        ],
        [
            873, 949, 947, 164, 149, 643, 286, 946, 832, 905, 167, 93, 336, 754, 679, 557, 894,
            893, 458, 591,
        ],
        [
            728, 629, 64, 470, 811, 778, 228, 910, 645, 121, 215, 324, 292, 112, 785, 462, 948, 62,
            140, 299,
        ],
        [
            432, 198, 788, 308, 390, 775, 92, 590, 349, 614, 340, 272, 220, 520, 779, 836, 409, 62,
            864, 390,
        ],
        [
            904, 322, 74, 511, 138, 406, 730, 285, 326, 212, 776, 813, 877, 718, 446, 251, 631,
            117, 448, 437,
        ],
        [
            684, 917, 260, 916, 259, 844, 95, 518, 613, 826, 162, 264, 412, 269, 114, 617, 271,
            446, 302, 996,
        ],
        [
            681, 98, 243, 528, 323, 211, 228, 770, 515, 906, 734, 723, 257, 866, 168, 630, 622,
            553, 789, 624,
        ],
        [
            321, 605, 758, 603, 824, 823, 317, 148, 77, 86, 466, 267, 429, 433, 325, 462, 892, 774,
            739, 935,
        ],
        [
            883, 339, 802, 793, 847, 472, 928, 474, 390, 291, 790, 891, 411, 918, 249, 315, 305,
            470, 690, 557,
        ],
        [
            554, 74, 417, 402, 516, 318, 217, 491, 171, 595, 268, 632, 821, 346, 942, 118, 442,
            504, 391, 551,
        ],
        [
            592, 946, 231, 155, 262, 146, 305, 407, 684, 911, 143, 924, 395, 8, 560, 838, 902, 611,
            735, 355,
        ],
        [
            822, 716, 888, 553, 619, 212, 347, 278, 878, 609, 883, 887, 305, 292, 732, 94, 12, 921,
            354, 172,
        ],
        [
            784, 906, 328, 205, 562, 827, 439, 335, 211, 792, 91, 838, 644, 307, 127, 649, 811,
            329, 302, 517,
        ],
        [
            513, 323, 132, 773, 452, 264, 725, 286, 854, 640, 115, 420, 257, 154, 780, 684, 107,
            684, 417, 775,
        ],
        [
            426, 213, 753, 469, 722, 325, 410, 603, 162, 100, 426, 842, 425, 86, 642, 555, 609,
            987, 414, 435,
        ],
        [
            120, 825, 351, 381, 216, 265, 947, 757, 754, 594, 948, 518, 423, 764, 866, 689, 719,
            317, 505, 744,
        ],
        [
            587, 343, 279, 680, 105, 885, 303, 207, 686, 276, 689, 844, 76, 112, 104, 144, 774,
            763, 849, 824,
        ],
        [
            67, 430, 263, 475, 466, 774, 275, 629, 411, 446, 127, 613, 123, 281, 907, 127, 308,
            492, 446, 319,
        ],
        [
            355, 632, 131, 197, 594, 440, 286, 800, 929, 849, 249, 129, 157, 424, 617, 16, 121,
            342, 718, 170,
        ],
        [
            621, 296, 128, 291, 409, 625, 382, 644, 876, 621, 82, 402, 648, 879, 828, 130, 221,
            566, 415, 723,
        ],
        [
            569, 724, 912, 619, 196, 66, 593, 156, 551, 908, 433, 463, 258, 295, 92, 251, 340, 221,
            821, 106,
        ],
        [
            763, 631, 819, 382, 169, 873, 352, 129, 287, 483, 467, 302, 679, 625, 425, 234, 630,
            451, 339, 330,
        ],
        [
            645, 289, 746, 849, 264, 279, 599, 123, 769, 615, 277, 108, 819, 624, 170, 422, 447,
            720, 772, 258,
        ],
        [
            403, 632, 114, 319, 682, 224, 906, 133, 53, 398, 384, 146, 425, 118, 288, 914, 132,
            299, 257, 836,
        ],
        [
            449, 150, 320, 386, 101, 348, 255, 138, 331, 632, 388, 729, 775, 414, 782, 761, 623,
            922, 840, 300,
        ],
        [
            826, 146, 318, 157, 776, 142, 260, 998, 53, 328, 745, 847, 629, 80, 641, 402, 868, 400,
            218, 470,
        ],
        [
            223, 839, 56, 839, 86, 176, 325, 214, 211, 890, 387, 721, 790, 514, 549, 635, 551, 846,
            783, 737,
        ],
        [
            863, 338, 165, 676, 141, 862, 261, 615, 122, 272, 942, 403, 143, 321, 588, 214, 579,
            107, 763, 282,
        ],
        [
            436, 194, 120, 286, 266, 554, 893, 615, 785, 352, 572, 472, 298, 399, 873, 756, 352,
            427, 819, 324,
        ],
        [
            311, 159, 780, 163, 815, 814, 823, 256, 893, 263, 917, 138, 327, 849, 909, 268, 821,
            922, 84, 912,
        ],
        [
            602, 826, 125, 137, 408, 294, 59, 915, 108, 277, 676, 945, 248, 869, 315, 210, 520,
            436, 267, 407,
        ],
        [
            313, 906, 61, 786, 692, 58, 157, 862, 724, 998, 721, 606, 307, 144, 510, 121, 944, 283,
            167, 724,
        ],
        [
            250, 296, 626, 597, 475, 334, 65, 589, 625, 893, 297, 638, 131, 66, 600, 132, 481, 731,
            553, 445,
        ],
        [
            681, 406, 560, 93, 984, 823, 343, 166, 452, 788, 311, 448, 121, 325, 790, 470, 355,
            872, 158, 126,
        ],
        [
            463, 947, 274, 217, 806, 115, 826, 107, 210, 621, 893, 288, 100, 903, 457, 612, 132,
            626, 826, 122,
        ],
        [
            893, 550, 983, 53, 917, 816, 838, 109, 299, 408, 406, 322, 874, 221, 161, 325, 642,
            388, 94, 102,
        ],
        [
            397, 279, 788, 463, 268, 233, 423, 604, 894, 640, 815, 893, 560, 76, 274, 675, 211,
            684, 240, 274,
        ],
        [
            687, 455, 825, 76, 716, 864, 520, 351, 769, 929, 472, 200, 786, 315, 812, 890, 279,
            891, 449, 394,
        ],
        [
            352, 730, 81, 742, 948, 148, 436, 338, 686, 103, 345, 596, 849, 717, 195, 462, 872,
            316, 480, 254,
        ],
        [
            710, 881, 230, 784, 491, 635, 144, 423, 449, 775, 234, 623, 882, 772, 304, 871, 465,
            745, 133, 113,
        ],
        [
            275, 92, 170, 392, 155, 821, 892, 406, 203, 409, 302, 66, 115, 634, 756, 773, 75, 720,
            840, 81,
        ],
        [
            267, 101, 431, 793, 678, 890, 453, 376, 864, 447, 76, 605, 555, 788, 599, 457, 339,
            124, 725, 595,
        ],
        [
            850, 942, 169, 873, 340, 398, 463, 636, 91, 681, 145, 676, 371, 280, 943, 455, 893,
            457, 422, 875,
        ],
        [
            873, 650, 89, 159, 79, 578, 611, 886, 554, 902, 247, 845, 595, 256, 840, 51, 318, 787,
            421, 345,
        ],
        [
            416, 80, 468, 736, 382, 295, 690, 636, 252, 604, 389, 232, 600, 276, 595, 352, 135,
            234, 912, 683,
        ],
        [
            868, 927, 719, 268, 312, 876, 153, 684, 740, 203, 78, 403, 398, 283, 553, 866, 640,
            760, 390, 147,
        ],
        [
            257, 263, 723, 872, 561, 837, 913, 274, 840, 680, 121, 588, 352, 55, 327, 252, 618, 52,
            948, 436,
        ],
        [
            82, 475, 337, 130, 491, 303, 633, 621, 770, 942, 274, 478, 682, 354, 758, 824, 232,
            731, 886, 328,
        ],
        [
            414, 616, 833, 141, 431, 118, 309, 332, 871, 397, 150, 868, 826, 215, 737, 907, 604,
            627, 93, 422,
        ],
        [
            304, 863, 471, 79, 522, 683, 508, 353, 908, 918, 516, 124, 418, 613, 382, 214, 422,
            774, 65, 117,
        ],
        [
            350, 643, 549, 409, 366, 649, 253, 885, 88, 417, 766, 221, 769, 331, 800, 683, 879,
            683, 142, 812,
        ],
        [
            180, 111, 948, 783, 948, 678, 171, 278, 550, 210, 311, 432, 78, 508, 330, 764, 416,
            198, 88, 382,
        ],
    ];
    let my_ticket = [157,59,163,149,83,131,107,89,109,113,151,53,127,97,79,103,101,173,167,61];
    (nearby_ticket,my_ticket)    
}
