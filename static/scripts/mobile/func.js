function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function mob_menu_hide() {
  document.querySelector(".window_fullscreen").style.display = "none";
  document.querySelector(".mob_menu").style.display = "block";
};
function getCookie(name) {
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        let c = cookies[i].trim().split('=');
        if (c[0] === name) {
            return c[1];
        }
    }
    return "";
}
function setCookie(name, value, days) {
    let cookie = `${name}=${encodeURIComponent(value)}`;
    if (days) {
        const expiry = new Date();
        expiry.setDate(expiry.getDate() + days);
        cookie += `; expires=${expiry.toUTCString()}`;
    }
    document.cookie = cookie + "; path=/";
};

function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("color");
    $link.href = href;
    $head.appendChild($link)
};
function get_custom_design() {
  color = "white";
  backgroud = getCookie("backgroud");
  if (backgroud != "") {
    color = backgroud;
  }
  addStyleSheets("/static/styles/color/" + color + ".css")
};
function check_first_load() {
  get_custom_design();
  span = document.body.querySelector(".span");
  if (window.location.search.split('?').length > 1) {
    span.innerHTML = "Permission Denied";
  }
  else if (!span.firstChild) {
    url = window.location.href;
    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=1", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        span.innerHTML = elem_.innerHTML;
        window.history.pushState({route: url}, "network", url);
        get_or_create_cookie_user();
        get_page_view_time(120);
        scrolled(document.body.querySelector(".span"));
      }
    }
    ajax_link.send();
  }
  else {
    get_active_button();
  }
}

function ajax_get_reload(url) {
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=2", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        rtr = document.getElementById('ajax');
        // статистика
        $link = document.location.pathname;
        meta_block = rtr.querySelector(".doc_title");
        if (meta_block.getAttribute("data-id")) {
          $object_id = meta_block.getAttribute("data-id");
        }
        else {
          $object_id = ""
        }
        $page_id = meta_block.getAttribute("page-id");
        $title = meta_block.getAttribute("data-title");
        //
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        sidebar = elem_.querySelector(".sidebar");
        rtr.innerHTML = elem_.innerHTML;
        window.scrollTo(0,0);
        document.title = rtr.querySelector(".doc_title").getAttribute("data-title");
        window.history.pushState({route: url}, "network", url);
        hide_nav_first_span();
        hide_nav_second_span();
        get_active_button();
        mob_menu_hide();
        get_page_view_time(120);
        scrolled(rtr);
        try {
          document.body.querySelector("#reload_nav_block").innerHTML = sidebar.innerHTML
        } catch { null };
        get_stat_meta($link, $title, $object_id, $page_id);
      }
    }
    ajax_link.send();
};

function deactivate_nav_buttons() {
  $mobile_nav = document.body.querySelector(".mobile_nav");
  buttons = $mobile_nav.querySelectorAll(".mobile_icon");
  for (var i = 0; i < buttons.length; i++){buttons[i].classList.remove("mobile_icon_active")};
};

function show_nav_first_span() {
  $mobile_nav = document.body.querySelector(".mobile_nav");
  first_span = $mobile_nav.previousElementSibling.previousElementSibling;
  first_span.style.display = "flex"; first_span.classList.add("btn_active");
  hide_nav_second_span();
  $mobile_nav.querySelector(".apps_btn").classList.add("mobile_icon_active");
  $mobile_nav.querySelector(".pages_btn").classList.remove("mobile_icon_active")
};
function hide_nav_first_span(){
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    first_span = $mobile_nav.previousElementSibling.previousElementSibling;
    first_span.style.display = "none"; first_span.classList.remove("btn_active");
    deactivate_nav_buttons();
    $mobile_nav.querySelector(".apps_btn").classList.remove("mobile_icon_active");
  } catch { null }
};
function toggle_nav_first_span(){
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    first_span = $mobile_nav.previousElementSibling.previousElementSibling;
    first_span.classList.contains("btn_active")
      ? (hide_nav_first_span(), first_span.classList.remove("btn_active"))
      : (show_nav_first_span(), first_span.classList.add("btn_active"))
  } catch { null }
};

function show_nav_second_span(){
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    second_span = $mobile_nav.previousElementSibling;
    second_span.style.display = "flex"; second_span.classList.add("btn_active");
    hide_nav_first_span();
    $mobile_nav.querySelector(".apps_btn").classList.remove("mobile_icon_active");
    $mobile_nav.querySelector(".pages_btn").classList.add("mobile_icon_active")
  } catch { null }
};
function hide_nav_second_span(){
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    second_span = $mobile_nav.previousElementSibling;
    second_span.style.display = "none"; second_span.classList.remove("btn_active");
    deactivate_nav_buttons();
    $mobile_nav.querySelector(".pages_btn").classList.remove("mobile_icon_active");
  } catch { null }
};
function toggle_nav_second_span(){
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    second_span = $mobile_nav.previousElementSibling;
    second_span.classList.contains("btn_active") ? (hide_nav_second_span(), second_span.classList.remove("btn_active")) : (show_nav_second_span(), second_span.classList.add("btn_active"))
  } catch { null }
};

function get_active_button() {
  try {
    $mobile_nav = document.body.querySelector(".mobile_nav");
    buttons = $mobile_nav.parentElement.querySelectorAll(".mobile_icon");
    path = document.location.pathname;
    for (var i = 0; i < buttons.length; i++){buttons[i].classList.remove("mobile_icon_current")};
    if (path == "/") {
        buttons[10].classList.add("mobile_icon_current");
    }
    else if (path.includes('service')) {
      buttons[0].classList.add("mobile_icon_current");
      buttons[11].classList.add("mobile_icon_current");
    }
    else if (path.includes('work')) {
      buttons[1].classList.add("mobile_icon_current");
      buttons[11].classList.add("mobile_icon_current");
    }
    else if (path.includes('store')) {
      buttons[2].classList.add("mobile_icon_current");
      buttons[11].classList.add("mobile_icon_current");
    }
    else if (path.includes('blog')) {
      buttons[3].classList.add("mobile_icon_current");
      buttons[11].classList.add("mobile_icon_current");
    }
    else if (path.includes('wiki')) {
      buttons[4].classList.add("mobile_icon_current");
      buttons[11].classList.add("mobile_icon_current");
    }
    else if (path == "/contacts/") {
      buttons[5].classList.add("mobile_icon_current");
      buttons[12].classList.add("mobile_icon_current");
    }
    else if (path == "/about/") {
      buttons[6].classList.add("mobile_icon_current");
      buttons[12].classList.add("mobile_icon_current");
    }
    else if (path == "/tags/") {
      buttons[7].classList.add("mobile_icon_current");
      buttons[12].classList.add("mobile_icon_current");
    }
    else if (path == "/search/") {
      buttons[8].classList.add("mobile_icon_current");
      buttons[12].classList.add("mobile_icon_current");
    }
    else if (path == "/auth/" || path.substr(1, 5) == "users") {
      buttons[13].classList.add("mobile_icon_current");
    }
  } catch { null }
};
