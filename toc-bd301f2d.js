// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="intro_1.html"><strong aria-hidden="true">1.</strong> RuSTy</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="build_and_install.html"><strong aria-hidden="true">2.</strong> Build &amp; Install</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty.html"><strong aria-hidden="true">3.</strong> Using RuSTy</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/build_configuration.html"><strong aria-hidden="true">3.1.</strong> Build Configuration</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.html"><strong aria-hidden="true">3.2.</strong> Error Configuration</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E001.html"><strong aria-hidden="true">3.2.1.</strong> E001</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E002.html"><strong aria-hidden="true">3.2.2.</strong> E002</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E003.html"><strong aria-hidden="true">3.2.3.</strong> E003</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E004.html"><strong aria-hidden="true">3.2.4.</strong> E004</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E005.html"><strong aria-hidden="true">3.2.5.</strong> E005</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E006.html"><strong aria-hidden="true">3.2.6.</strong> E006</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E007.html"><strong aria-hidden="true">3.2.7.</strong> E007</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E008.html"><strong aria-hidden="true">3.2.8.</strong> E008</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E009.html"><strong aria-hidden="true">3.2.9.</strong> E009</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E010.html"><strong aria-hidden="true">3.2.10.</strong> E010</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E011.html"><strong aria-hidden="true">3.2.11.</strong> E011</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E012.html"><strong aria-hidden="true">3.2.12.</strong> E012</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E013.html"><strong aria-hidden="true">3.2.13.</strong> E013</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E014.html"><strong aria-hidden="true">3.2.14.</strong> E014</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E015.html"><strong aria-hidden="true">3.2.15.</strong> E015</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E016.html"><strong aria-hidden="true">3.2.16.</strong> E016</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E017.html"><strong aria-hidden="true">3.2.17.</strong> E017</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E018.html"><strong aria-hidden="true">3.2.18.</strong> E018</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E019.html"><strong aria-hidden="true">3.2.19.</strong> E019</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E020.html"><strong aria-hidden="true">3.2.20.</strong> E020</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E021.html"><strong aria-hidden="true">3.2.21.</strong> E021</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E022.html"><strong aria-hidden="true">3.2.22.</strong> E022</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E023.html"><strong aria-hidden="true">3.2.23.</strong> E023</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E024.html"><strong aria-hidden="true">3.2.24.</strong> E024</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E025.html"><strong aria-hidden="true">3.2.25.</strong> E025</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E026.html"><strong aria-hidden="true">3.2.26.</strong> E026</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E027.html"><strong aria-hidden="true">3.2.27.</strong> E027</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E028.html"><strong aria-hidden="true">3.2.28.</strong> E028</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E029.html"><strong aria-hidden="true">3.2.29.</strong> E029</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E030.html"><strong aria-hidden="true">3.2.30.</strong> E030</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E031.html"><strong aria-hidden="true">3.2.31.</strong> E031</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E032.html"><strong aria-hidden="true">3.2.32.</strong> E032</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E033.html"><strong aria-hidden="true">3.2.33.</strong> E033</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E034.html"><strong aria-hidden="true">3.2.34.</strong> E034</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E035.html"><strong aria-hidden="true">3.2.35.</strong> E035</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E036.html"><strong aria-hidden="true">3.2.36.</strong> E036</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E037.html"><strong aria-hidden="true">3.2.37.</strong> E037</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E038.html"><strong aria-hidden="true">3.2.38.</strong> E038</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E039.html"><strong aria-hidden="true">3.2.39.</strong> E039</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E040.html"><strong aria-hidden="true">3.2.40.</strong> E040</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E041.html"><strong aria-hidden="true">3.2.41.</strong> E041</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E042.html"><strong aria-hidden="true">3.2.42.</strong> E042</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E043.html"><strong aria-hidden="true">3.2.43.</strong> E043</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E044.html"><strong aria-hidden="true">3.2.44.</strong> E044</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E045.html"><strong aria-hidden="true">3.2.45.</strong> E045</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E046.html"><strong aria-hidden="true">3.2.46.</strong> E046</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E047.html"><strong aria-hidden="true">3.2.47.</strong> E047</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E048.html"><strong aria-hidden="true">3.2.48.</strong> E048</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E049.html"><strong aria-hidden="true">3.2.49.</strong> E049</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E050.html"><strong aria-hidden="true">3.2.50.</strong> E050</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E051.html"><strong aria-hidden="true">3.2.51.</strong> E051</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E052.html"><strong aria-hidden="true">3.2.52.</strong> E052</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E053.html"><strong aria-hidden="true">3.2.53.</strong> E053</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E054.html"><strong aria-hidden="true">3.2.54.</strong> E054</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E055.html"><strong aria-hidden="true">3.2.55.</strong> E055</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E056.html"><strong aria-hidden="true">3.2.56.</strong> E056</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E057.html"><strong aria-hidden="true">3.2.57.</strong> E057</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E058.html"><strong aria-hidden="true">3.2.58.</strong> E058</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E059.html"><strong aria-hidden="true">3.2.59.</strong> E059</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E060.html"><strong aria-hidden="true">3.2.60.</strong> E060</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E061.html"><strong aria-hidden="true">3.2.61.</strong> E061</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E062.html"><strong aria-hidden="true">3.2.62.</strong> E062</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E063.html"><strong aria-hidden="true">3.2.63.</strong> E063</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E064.html"><strong aria-hidden="true">3.2.64.</strong> E064</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E065.html"><strong aria-hidden="true">3.2.65.</strong> E065</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E066.html"><strong aria-hidden="true">3.2.66.</strong> E066</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E067.html"><strong aria-hidden="true">3.2.67.</strong> E067</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E068.html"><strong aria-hidden="true">3.2.68.</strong> E068</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E069.html"><strong aria-hidden="true">3.2.69.</strong> E069</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E070.html"><strong aria-hidden="true">3.2.70.</strong> E070</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E071.html"><strong aria-hidden="true">3.2.71.</strong> E071</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E072.html"><strong aria-hidden="true">3.2.72.</strong> E072</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E073.html"><strong aria-hidden="true">3.2.73.</strong> E073</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E074.html"><strong aria-hidden="true">3.2.74.</strong> E074</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E075.html"><strong aria-hidden="true">3.2.75.</strong> E075</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E076.html"><strong aria-hidden="true">3.2.76.</strong> E076</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E077.html"><strong aria-hidden="true">3.2.77.</strong> E077</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E078.html"><strong aria-hidden="true">3.2.78.</strong> E078</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E079.html"><strong aria-hidden="true">3.2.79.</strong> E079</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E080.html"><strong aria-hidden="true">3.2.80.</strong> E080</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E081.html"><strong aria-hidden="true">3.2.81.</strong> E081</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E082.html"><strong aria-hidden="true">3.2.82.</strong> E082</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E083.html"><strong aria-hidden="true">3.2.83.</strong> E083</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E084.html"><strong aria-hidden="true">3.2.84.</strong> E084</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E085.html"><strong aria-hidden="true">3.2.85.</strong> E085</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E086.html"><strong aria-hidden="true">3.2.86.</strong> E086</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E087.html"><strong aria-hidden="true">3.2.87.</strong> E087</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E088.html"><strong aria-hidden="true">3.2.88.</strong> E088</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E089.html"><strong aria-hidden="true">3.2.89.</strong> E089</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E090.html"><strong aria-hidden="true">3.2.90.</strong> E090</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E091.html"><strong aria-hidden="true">3.2.91.</strong> E091</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E092.html"><strong aria-hidden="true">3.2.92.</strong> E092</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E093.html"><strong aria-hidden="true">3.2.93.</strong> E093</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E094.html"><strong aria-hidden="true">3.2.94.</strong> E094</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E095.html"><strong aria-hidden="true">3.2.95.</strong> E095</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E096.html"><strong aria-hidden="true">3.2.96.</strong> E096</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E097.html"><strong aria-hidden="true">3.2.97.</strong> E097</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E098.html"><strong aria-hidden="true">3.2.98.</strong> E098</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E099.html"><strong aria-hidden="true">3.2.99.</strong> E099</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E100.html"><strong aria-hidden="true">3.2.100.</strong> E100</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E101.html"><strong aria-hidden="true">3.2.101.</strong> E101</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E102.html"><strong aria-hidden="true">3.2.102.</strong> E102</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E103.html"><strong aria-hidden="true">3.2.103.</strong> E103</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E104.html"><strong aria-hidden="true">3.2.104.</strong> E104</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E105.html"><strong aria-hidden="true">3.2.105.</strong> E105</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E106.html"><strong aria-hidden="true">3.2.106.</strong> E106</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E107.html"><strong aria-hidden="true">3.2.107.</strong> E107</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E108.html"><strong aria-hidden="true">3.2.108.</strong> E108</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E109.html"><strong aria-hidden="true">3.2.109.</strong> E109</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E110.html"><strong aria-hidden="true">3.2.110.</strong> E110</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E111.html"><strong aria-hidden="true">3.2.111.</strong> E111</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E112.html"><strong aria-hidden="true">3.2.112.</strong> E112</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E113.html"><strong aria-hidden="true">3.2.113.</strong> E113</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E114.html"><strong aria-hidden="true">3.2.114.</strong> E114</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E115.html"><strong aria-hidden="true">3.2.115.</strong> E115</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E116.html"><strong aria-hidden="true">3.2.116.</strong> E116</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E117.html"><strong aria-hidden="true">3.2.117.</strong> E117</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E118.html"><strong aria-hidden="true">3.2.118.</strong> E118</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E119.html"><strong aria-hidden="true">3.2.119.</strong> E119</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E120.html"><strong aria-hidden="true">3.2.120.</strong> E120</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E121.html"><strong aria-hidden="true">3.2.121.</strong> E121</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/error_configuration.md/E122.html"><strong aria-hidden="true">3.2.122.</strong> E122</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="using_rusty/header_generation.html"><strong aria-hidden="true">3.3.</strong> Header Generation</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">4.</strong> Writing ST Programs</span><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="libraries.html"><strong aria-hidden="true">4.1.</strong> Libraries</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="libraries/external_functions.html"><strong aria-hidden="true">4.1.1.</strong> External Functions</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="libraries/api_lib_guide.html"><strong aria-hidden="true">4.1.2.</strong> API guidelines</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">4.2.</strong> Using in external programs</span></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="pous.html"><strong aria-hidden="true">5.</strong> POUs</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="variables.html"><strong aria-hidden="true">6.</strong> Variables</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="datatypes.html"><strong aria-hidden="true">7.</strong> Datatypes</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="direct_variables.html"><strong aria-hidden="true">8.</strong> Direct Bit Access</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/architecture.html"><strong aria-hidden="true">9.</strong> Architecture</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/parser.html"><strong aria-hidden="true">9.1.</strong> Parser</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/indexer.html"><strong aria-hidden="true">9.2.</strong> Indexer &amp; Symbol-Table</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/linker.html"><strong aria-hidden="true">9.3.</strong> Linker</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/validation.html"><strong aria-hidden="true">9.4.</strong> Validation</a></span></li><li class="chapter-item "><span class="chapter-link-wrapper"><a href="arch/codegen.html"><strong aria-hidden="true">9.5.</strong> Codegen</a></span></li></ol><li class="chapter-item "><span class="chapter-link-wrapper"><a href="cfc/cfc.html"><strong aria-hidden="true">10.</strong> CFC</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item "><span class="chapter-link-wrapper"><a href="cfc/m2m.html"><strong aria-hidden="true">10.1.</strong> Model-to-Model Conversion</a></span></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

