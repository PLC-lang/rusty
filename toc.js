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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item "><a href="intro_1.html"><strong aria-hidden="true">1.</strong> RuSTy</a></li><li class="chapter-item "><a href="build_and_install.html"><strong aria-hidden="true">2.</strong> Build &amp; Install</a></li><li class="chapter-item "><a href="using_rusty.html"><strong aria-hidden="true">3.</strong> Using RuSTy</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="using_rusty/build_configuration.html"><strong aria-hidden="true">3.1.</strong> Build Configuration</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.html"><strong aria-hidden="true">3.2.</strong> Error Configuration</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="using_rusty/error_configuration.md/E001.html"><strong aria-hidden="true">3.2.1.</strong> E001</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E002.html"><strong aria-hidden="true">3.2.2.</strong> E002</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E003.html"><strong aria-hidden="true">3.2.3.</strong> E003</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E004.html"><strong aria-hidden="true">3.2.4.</strong> E004</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E005.html"><strong aria-hidden="true">3.2.5.</strong> E005</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E006.html"><strong aria-hidden="true">3.2.6.</strong> E006</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E007.html"><strong aria-hidden="true">3.2.7.</strong> E007</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E008.html"><strong aria-hidden="true">3.2.8.</strong> E008</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E009.html"><strong aria-hidden="true">3.2.9.</strong> E009</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E010.html"><strong aria-hidden="true">3.2.10.</strong> E010</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E011.html"><strong aria-hidden="true">3.2.11.</strong> E011</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E012.html"><strong aria-hidden="true">3.2.12.</strong> E012</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E013.html"><strong aria-hidden="true">3.2.13.</strong> E013</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E014.html"><strong aria-hidden="true">3.2.14.</strong> E014</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E015.html"><strong aria-hidden="true">3.2.15.</strong> E015</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E016.html"><strong aria-hidden="true">3.2.16.</strong> E016</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E017.html"><strong aria-hidden="true">3.2.17.</strong> E017</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E018.html"><strong aria-hidden="true">3.2.18.</strong> E018</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E019.html"><strong aria-hidden="true">3.2.19.</strong> E019</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E020.html"><strong aria-hidden="true">3.2.20.</strong> E020</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E021.html"><strong aria-hidden="true">3.2.21.</strong> E021</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E022.html"><strong aria-hidden="true">3.2.22.</strong> E022</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E023.html"><strong aria-hidden="true">3.2.23.</strong> E023</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E024.html"><strong aria-hidden="true">3.2.24.</strong> E024</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E025.html"><strong aria-hidden="true">3.2.25.</strong> E025</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E026.html"><strong aria-hidden="true">3.2.26.</strong> E026</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E027.html"><strong aria-hidden="true">3.2.27.</strong> E027</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E028.html"><strong aria-hidden="true">3.2.28.</strong> E028</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E029.html"><strong aria-hidden="true">3.2.29.</strong> E029</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E030.html"><strong aria-hidden="true">3.2.30.</strong> E030</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E031.html"><strong aria-hidden="true">3.2.31.</strong> E031</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E032.html"><strong aria-hidden="true">3.2.32.</strong> E032</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E033.html"><strong aria-hidden="true">3.2.33.</strong> E033</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E034.html"><strong aria-hidden="true">3.2.34.</strong> E034</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E035.html"><strong aria-hidden="true">3.2.35.</strong> E035</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E036.html"><strong aria-hidden="true">3.2.36.</strong> E036</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E037.html"><strong aria-hidden="true">3.2.37.</strong> E037</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E038.html"><strong aria-hidden="true">3.2.38.</strong> E038</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E039.html"><strong aria-hidden="true">3.2.39.</strong> E039</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E040.html"><strong aria-hidden="true">3.2.40.</strong> E040</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E041.html"><strong aria-hidden="true">3.2.41.</strong> E041</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E042.html"><strong aria-hidden="true">3.2.42.</strong> E042</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E043.html"><strong aria-hidden="true">3.2.43.</strong> E043</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E044.html"><strong aria-hidden="true">3.2.44.</strong> E044</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E045.html"><strong aria-hidden="true">3.2.45.</strong> E045</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E046.html"><strong aria-hidden="true">3.2.46.</strong> E046</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E047.html"><strong aria-hidden="true">3.2.47.</strong> E047</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E048.html"><strong aria-hidden="true">3.2.48.</strong> E048</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E049.html"><strong aria-hidden="true">3.2.49.</strong> E049</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E050.html"><strong aria-hidden="true">3.2.50.</strong> E050</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E051.html"><strong aria-hidden="true">3.2.51.</strong> E051</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E052.html"><strong aria-hidden="true">3.2.52.</strong> E052</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E053.html"><strong aria-hidden="true">3.2.53.</strong> E053</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E054.html"><strong aria-hidden="true">3.2.54.</strong> E054</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E055.html"><strong aria-hidden="true">3.2.55.</strong> E055</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E056.html"><strong aria-hidden="true">3.2.56.</strong> E056</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E057.html"><strong aria-hidden="true">3.2.57.</strong> E057</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E058.html"><strong aria-hidden="true">3.2.58.</strong> E058</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E059.html"><strong aria-hidden="true">3.2.59.</strong> E059</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E060.html"><strong aria-hidden="true">3.2.60.</strong> E060</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E061.html"><strong aria-hidden="true">3.2.61.</strong> E061</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E062.html"><strong aria-hidden="true">3.2.62.</strong> E062</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E063.html"><strong aria-hidden="true">3.2.63.</strong> E063</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E064.html"><strong aria-hidden="true">3.2.64.</strong> E064</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E065.html"><strong aria-hidden="true">3.2.65.</strong> E065</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E066.html"><strong aria-hidden="true">3.2.66.</strong> E066</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E067.html"><strong aria-hidden="true">3.2.67.</strong> E067</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E068.html"><strong aria-hidden="true">3.2.68.</strong> E068</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E069.html"><strong aria-hidden="true">3.2.69.</strong> E069</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E070.html"><strong aria-hidden="true">3.2.70.</strong> E070</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E071.html"><strong aria-hidden="true">3.2.71.</strong> E071</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E072.html"><strong aria-hidden="true">3.2.72.</strong> E072</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E073.html"><strong aria-hidden="true">3.2.73.</strong> E073</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E074.html"><strong aria-hidden="true">3.2.74.</strong> E074</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E075.html"><strong aria-hidden="true">3.2.75.</strong> E075</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E076.html"><strong aria-hidden="true">3.2.76.</strong> E076</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E077.html"><strong aria-hidden="true">3.2.77.</strong> E077</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E078.html"><strong aria-hidden="true">3.2.78.</strong> E078</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E079.html"><strong aria-hidden="true">3.2.79.</strong> E079</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E080.html"><strong aria-hidden="true">3.2.80.</strong> E080</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E081.html"><strong aria-hidden="true">3.2.81.</strong> E081</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E082.html"><strong aria-hidden="true">3.2.82.</strong> E082</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E083.html"><strong aria-hidden="true">3.2.83.</strong> E083</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E084.html"><strong aria-hidden="true">3.2.84.</strong> E084</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E085.html"><strong aria-hidden="true">3.2.85.</strong> E085</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E086.html"><strong aria-hidden="true">3.2.86.</strong> E086</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E087.html"><strong aria-hidden="true">3.2.87.</strong> E087</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E088.html"><strong aria-hidden="true">3.2.88.</strong> E088</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E089.html"><strong aria-hidden="true">3.2.89.</strong> E089</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E090.html"><strong aria-hidden="true">3.2.90.</strong> E090</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E091.html"><strong aria-hidden="true">3.2.91.</strong> E091</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E092.html"><strong aria-hidden="true">3.2.92.</strong> E092</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E093.html"><strong aria-hidden="true">3.2.93.</strong> E093</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E094.html"><strong aria-hidden="true">3.2.94.</strong> E094</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E095.html"><strong aria-hidden="true">3.2.95.</strong> E095</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E096.html"><strong aria-hidden="true">3.2.96.</strong> E096</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E097.html"><strong aria-hidden="true">3.2.97.</strong> E097</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E098.html"><strong aria-hidden="true">3.2.98.</strong> E098</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E099.html"><strong aria-hidden="true">3.2.99.</strong> E099</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E100.html"><strong aria-hidden="true">3.2.100.</strong> E100</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E101.html"><strong aria-hidden="true">3.2.101.</strong> E101</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E102.html"><strong aria-hidden="true">3.2.102.</strong> E102</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E103.html"><strong aria-hidden="true">3.2.103.</strong> E103</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E104.html"><strong aria-hidden="true">3.2.104.</strong> E104</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E105.html"><strong aria-hidden="true">3.2.105.</strong> E105</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E106.html"><strong aria-hidden="true">3.2.106.</strong> E106</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E107.html"><strong aria-hidden="true">3.2.107.</strong> E107</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E108.html"><strong aria-hidden="true">3.2.108.</strong> E108</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E109.html"><strong aria-hidden="true">3.2.109.</strong> E109</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E110.html"><strong aria-hidden="true">3.2.110.</strong> E110</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E111.html"><strong aria-hidden="true">3.2.111.</strong> E111</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E112.html"><strong aria-hidden="true">3.2.112.</strong> E112</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E113.html"><strong aria-hidden="true">3.2.113.</strong> E113</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E114.html"><strong aria-hidden="true">3.2.114.</strong> E114</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E115.html"><strong aria-hidden="true">3.2.115.</strong> E115</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E116.html"><strong aria-hidden="true">3.2.116.</strong> E116</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E117.html"><strong aria-hidden="true">3.2.117.</strong> E117</a></li><li class="chapter-item "><a href="using_rusty/error_configuration.md/E118.html"><strong aria-hidden="true">3.2.118.</strong> E118</a></li></ol></li></ol></li><li class="chapter-item "><div><strong aria-hidden="true">4.</strong> Writing ST Programs</div><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="libraries.html"><strong aria-hidden="true">4.1.</strong> Libraries</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="libraries/external_functions.html"><strong aria-hidden="true">4.1.1.</strong> External Functions</a></li><li class="chapter-item "><a href="libraries/api_lib_guide.html"><strong aria-hidden="true">4.1.2.</strong> API guidelines</a></li></ol></li><li class="chapter-item "><div><strong aria-hidden="true">4.2.</strong> Using in external programs</div></li></ol></li><li class="chapter-item "><a href="pous.html"><strong aria-hidden="true">5.</strong> POUs</a></li><li class="chapter-item "><a href="variables.html"><strong aria-hidden="true">6.</strong> Variables</a></li><li class="chapter-item "><a href="datatypes.html"><strong aria-hidden="true">7.</strong> Datatypes</a></li><li class="chapter-item "><a href="direct_variables.html"><strong aria-hidden="true">8.</strong> Direct Bit Access</a></li><li class="chapter-item "><a href="arch/architecture.html"><strong aria-hidden="true">9.</strong> Architecture</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="arch/parser.html"><strong aria-hidden="true">9.1.</strong> Parser</a></li><li class="chapter-item "><a href="arch/indexer.html"><strong aria-hidden="true">9.2.</strong> Indexer &amp; Symbol-Table</a></li><li class="chapter-item "><a href="arch/linker.html"><strong aria-hidden="true">9.3.</strong> Linker</a></li><li class="chapter-item "><a href="arch/validation.html"><strong aria-hidden="true">9.4.</strong> Validation</a></li><li class="chapter-item "><a href="arch/codegen.html"><strong aria-hidden="true">9.5.</strong> Codegen</a></li></ol></li><li class="chapter-item "><a href="cfc/cfc.html"><strong aria-hidden="true">10.</strong> CFC</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="cfc/m2m.html"><strong aria-hidden="true">10.1.</strong> Model-to-Model Conversion</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
