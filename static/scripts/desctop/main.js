function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};
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
on('body', 'mouseover', '.mn-has-sub', function(event) {
  this.nextElementSibling.style.display = "block"
});
on('body', 'mouseover', '.mn-sub', function(event) {
  this.style.display = "block"
});
on('body', 'mouseout', '.mn-has-sub', function(event) {
  this.nextElementSibling.style.display = "none"
});
on('body', 'mouseout', '.mn-sub', function(event) {
  this.style.display = "none"
});

function get_active_button() {
  try {
    $nav = document.body.querySelector(".desktop-nav");
    links = $nav.querySelectorAll(".folder");
    path = document.location.pathname;
    for (var i = 0; i < links.length; i++){links[i].classList.remove("active")};
    if (path == "/") {
        $nav.querySelector(".main").classList.add("active");
    }
    else if (path.includes('service')) {
      $nav.querySelector(".service").classList.add("active");
    }
    else if (path.includes('work')) {
      $nav.querySelector(".work").classList.add("active");
    }
    else if (path.includes('store')) {
      $nav.querySelector(".store").classList.add("active");
    }
    else if (path.includes('blog')) {
      $nav.querySelector(".blog").classList.add("active");
    }
    else if (path.includes('wiki')) {
      $nav.querySelector(".wiki").classList.add("active");
    }
    else if (path == "/info/") {
      $nav.querySelector(".infos").classList.add("active");
    }
    else if (path == "/tags/") {
      $nav.querySelector(".tags").classList.add("active");
    }
    else if (path.includes("/search/")) {
      $nav.querySelector(".search").classList.add("active");
    }
    else if (path == "/login/") {
      $nav.querySelector(".login").classList.add("active");
    }
    else if (path == "/auth/") {
      $nav.querySelector(".auth").classList.add("active");
    }
  } catch { null }
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
  span = document.body.querySelector(".span");
  console.log(window.location.search.split('?').length);
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
        get_custom_design();
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        span.innerHTML = elem_.innerHTML;
        get_or_create_cookie_user();
        get_active_button();
        get_page_view_time(120);
        scrolled(document.body.querySelector(".span"));
        //window.history.pushState({route: url}, "network", url);
      }
    }
    ajax_link.send();
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
        document.title = rtr.querySelector(".doc_title").getAttribute("data-title");
        window.scrollTo(0,0);
        window.history.pushState(null, "ajax_reload", url);
        get_active_button();
        get_page_view_time(120);
        scrolled(rtr);
        get_stat_meta($link, $title, $object_id, $page_id);
      }
    }
    ajax_link.send();
};

on('body', 'input', '.desctop_folder_search', function() {
    _this = this;
    value = _this.value;
    parent = _this.parentElement.parentElement.parentElement.parentElement.parentElement;
    content_block = parent.querySelector(".content");
    search_block = content_block.previousElementSibling;
    if (value == "") {
      search_block.innerHTML= "";
      content_block.classList.remove("hidden");
      return;
    }
    if (_this.getAttribute("data-folder")) {
      folder = _this.getAttribute("data-folder")
    } else {
      folder = ""
    };
    url = "/search" + folder + "/" + _this.value + "/";

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=1", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        search_block.innerHTML = elem_.querySelector(".search_section").innerHTML;
        content_block.classList.add("hidden");
      }
    }
    ajax_link.send();
});

check_first_load();
