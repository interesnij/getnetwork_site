function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("color");
    $link.href = href;
    $head.appendChild($link)
};
function setEndOfContenteditable(contentEditableElement) {
    var range,selection;
    if(document.createRange) {
        range = document.createRange();
        range.selectNodeContents(contentEditableElement);
        range.collapse(false);
        selection = window.getSelection();
        selection.removeAllRanges();
        selection.addRange(range);
    }
    else if(document.selection) {
        range = document.body.createTextRange();
        range.moveToElementText(contentEditableElement);
        range.collapse(false);
        range.select();
    }
};
function init_wow(){
    var wow = new WOW({
      boxClass: 'wow',
      animateClass: 'animated',
      offset: 90,
      mobile: false,
      live: true
  });
  wow.init();
};

class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('#ajax');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.style.display = 'none';
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.style.display = 'none';
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
}

function toast_success(text) {
    var toasts = new ToastManager();
    toasts.showSuccess(text)
}

function toast_error(text) {
    var toasts = new ToastManager();
    toasts.showError(text)
}

function toast_info(text) {
    var toasts = new ToastManager();
    toasts.showInfo(text)
}

function toast_warning(text) {
    var toasts = new ToastManager();
    toasts.showWarning(text)
}

function service_tab_action(is, tab_class){
  if (!is.classList.contains("active")){
  nav = is.parentElement;
  nav_items = nav.querySelectorAll(".yy");
  for (var i = 0; i < nav_items.length; i++){nav_items[i].classList.remove("active"); nav_items[i].classList.add("pointer")};
  is.classList.add("active"); is.classList.remove("pointer");
  tabs = nav.nextElementSibling;
  tabs_items = tabs.querySelectorAll(".tab-pane");
  for (var i = 0; i < tabs_items.length; i++){tabs_items[i].classList.remove("active")};
  cur = tabs.querySelector(tab_class);
  cur.classList.add("active")
}};

function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function format_text(text) {
  br = text.querySelectorAll("br");
  img = text.querySelectorAll("img");
  p = text.querySelectorAll("p");
  ol = text.querySelectorAll("ol");
  ul = text.querySelectorAll("ul");
  a = text.querySelectorAll("a");
  h1 = text.querySelectorAll("h1");
  h2 = text.querySelectorAll("h2");
  h3 = text.querySelectorAll("h3");
  h4 = text.querySelectorAll("h4");
  h5 = text.querySelectorAll("h5");
  h6 = text.querySelectorAll("h6");
  span = text.querySelectorAll("span");

  for (var i = 0; i < br.length; i++){
      br[i].removeAttribute("style"); br[i].className = ''
  };
  for (var i = 0; i < img.length; i++){
      img[i].removeAttribute("style"); img[i].className = ''
  };
  for (var i = 0; i < p.length; i++){
      p[i].removeAttribute("style"); p[i].className = ''
  };
  for (var i = 0; i < ul.length; i++){
      ul[i].removeAttribute("style"); ul[i].className = ''
  };
  for (var i = 0; i < ol.length; i++){
      ol[i].removeAttribute("style"); ol[i].className = ''
  };
  for (var i = 0; i < a.length; i++){
      a[i].removeAttribute("style"); a[i].className = ''
  };
  for (var i = 0; i < span.length; i++){
      span[i].removeAttribute("style"); span[i].className = ''
  };
  for (var i = 0; i < h1.length; i++){
      h1[i].removeAttribute("style"); h1[i].className = ''
  };
  for (var i = 0; i < h2.length; i++){
      h2[i].removeAttribute("style"); h2[i].className = ''
  };
  for (var i = 0; i < h3.length; i++){
      h3[i].removeAttribute("style"); h3[i].className = ''
  };
  for (var i = 0; i < h4.length; i++){
      h4[i].removeAttribute("style"); h4[i].className = ''
  };
  for (var i = 0; i < h5.length; i++){
      h5[i].removeAttribute("style"); h5[i].className = ''
  };
  for (var i = 0; i < h6.length; i++){
      h6[i].removeAttribute("style"); h6[i].className = ''
  };

  return text
};

on('#ajax', 'input', '.smile_supported', function() {
//  setTimeout(function(){
    //setEndOfContenteditable(this)
//  }, 2000)
});

function send_content_data(url) {
  text_val = document.body.querySelector(".smile_supported");
  _val = format_text(text_val);
  _text = _val.innerHTML;

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url + "?content=" + _text, true );
  link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url)
  } else { console.log(link.responseText) }};
  link.send();
};

function send_post_data(form, url) {
  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url)
  } else { console.log(link.responseText) }};
  link.send(form_data);
};
function delete_item(url) {
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    toast_success("Удалено!");
  }};
  link.send();
}

function get_custom_design() {
  params = window.location.search.replace( '?', '').split('&');
  if (params) {
    console.log(params);
    if (params[0].split("=")[0] == "f") {
      color = params[0].split("=")[1];

      if (color == "white") {
        addStyleSheets("/static/styles/color/white.css")
      } else if (color == "dark") {
        addStyleSheets("/static/styles/color/dark.css")
      } else if (color == "yellow") {
        addStyleSheets("/static/styles/color/yellow.css")
      } else if (color == "white_kletka") {
        addStyleSheets("/static/styles/color/white_kletka.css")
      } else if (color == "dark_wood") {
        addStyleSheets("/static/styles/color/dark_wood.css")
      };
      btn = document.body.querySelector(".anon_color_change");
      btn.setAttribute("data-color", color)
    };
  };
};
get_custom_design();

on('body', 'click', '.ajax', function(event) {
  //this.classList.remove("ajax");
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.pathname){
    ajax_get_reload(url);
  } else {toast_info("Вы уже на этой странице")}
});

/////////////////////////////
on('body', 'click', '#create_work_btn', function() {
  send_post_data(this.parentElement, "/create_work/");
});
on('body', 'click', '#create_work_category_btn', function() {
  send_post_data(this.parentElement, "/create_work_categories/");
});
on('body', 'click', '#edit_work_btn', function() {
  send_post_data(this.parentElement, "/edit_work/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_work_category_btn', function() {
  send_post_data(this.parentElement, "/edit_work_categories/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_work', function() {
  delete_item("/delete_work/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_work_category', function() {
  delete_item("/delete_work_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

////////////////////////////////////

on('body', 'click', '#create_blog_btn', function() {
  send_post_data(this.parentElement, "/create_blog/");
});
on('body', 'click', '#edit_blog_btn', function() {
  send_post_data(this.parentElement, "/edit_blog/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_blog_btn', function() {
  send_content_data("/edit_content_blog/" + this.getAttribute("data-pk") + "/");
});

on('body', 'click', '#create_blog_category_btn', function() {
  send_post_data(this.parentElement, "/create_blog_categories/");
});
on('body', 'click', '#edit_blog_category_btn', function() {
  send_post_data(this.parentElement, "/edit_blog_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_blog', function() {
  delete_item("/delete_blog/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_blog_category', function() {
  delete_item("/delete_blog_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

/////////////////////////////////////////

on('body', 'click', '#create_wiki_btn', function() {
  send_post_data(this.parentElement, "/create_wiki/");
});
on('body', 'click', '#create_wiki_category_btn', function() {
  send_post_data(this.parentElement, "/create_wiki_categories/");
});
on('body', 'click', '#edit_wiki_btn', function() {
  send_post_data(this.parentElement, "/edit_wiki/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_wiki_category_btn', function() {
  send_post_data(this.parentElement, "/edit_wiki_categories/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_wiki', function() {
  delete_item("/delete_wiki/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_wiki_category', function() {
  delete_item("/delete_wiki_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

 //////////////////////////////////

on('body', 'click', '#create_store_btn', function() {
  send_post_data(this.parentElement, "/create_store/");
});
on('body', 'click', '#create_store_category_btn', function() {
  send_post_data(this.parentElement, "/create_store_categories/");
});
on('body', 'click', '#edit_store_btn', function() {
  send_post_data(this.parentElement, "/edit_store/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_store_category_btn', function() {
  send_post_data(this.parentElement, "/edit_store_categories/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_store', function() {
  delete_item("/delete_store/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_store_category', function() {
  delete_item("/delete_store_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

////////////////////////////////////

on('body', 'click', '#create_service_btn', function() {
  send_post_data(this.parentElement, "/create_service/");
});
on('body', 'click', '#create_service_category_btn', function() {
  send_post_data(this.parentElement, "/create_service_categories/");
});
on('body', 'click', '#edit_service_btn', function() {
  send_post_data(this.parentElement, "/edit_service/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_service_category_btn', function() {
  send_post_data(this.parentElement, "/edit_service_categories/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_service', function() {
  delete_item("/delete_service/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_service_category', function() {
  delete_item("/delete_service_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

///////////////////////////

on('body', 'click', '#create_tag_btn', function() {
  send_post_data(this.parentElement, "/create_tag/");
});
on('body', 'click', '#edit_tag_btn', function() {
  send_post_data(this.parentElement, "/edit_tag/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_tag', function() {
  delete_item("/delete_tag/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

init_wow();

on('#ajax', 'click', '.s_1', function() {
  service_tab_action(this, ".tab_1")
});
on('#ajax', 'click', '.s_2', function() {
  service_tab_action(this, ".tab_2")
});
on('#ajax', 'click', '.s_3', function() {
  service_tab_action(this, ".tab_3")
});
on('#ajax', 'click', '.s_4', function() {
  service_tab_action(this, ".tab_4")
});
on('#ajax', 'click', '.s_5', function() {
  service_tab_action(this, ".tab_5")
});
on('#ajax', 'click', '.s_6', function() {
  service_tab_action(this, ".tab_6")
});
on('#ajax', 'click', '.s_7', function() {
  service_tab_action(this, ".tab_7")
});

on('body', 'click', '.anon_color_change', function() {
  window.history.replaceState(null, null, window.location.pathname);
  color = this.getAttribute("data-color");
  if (color == "white") {
    addStyleSheets("/static/styles/color/dark.css");
    this.setAttribute("data-color", "dark");
    new_color = "dark"
  } else if (color == "dark") {
    addStyleSheets("/static/styles/color/yellow.css");
    this.setAttribute("data-color", "yellow");
    new_color = "yellow"
  } else if (color == "yellow") {
    addStyleSheets("/static/styles/color/white_kletka.css");
    this.setAttribute("data-color", "white_kletka");
    new_color = "white_kletka"
  } else if (color == "white_kletka") {
    addStyleSheets("/static/styles/color/dark_wood.css");
    this.setAttribute("data-color", "dark_wood");
    new_color = "dark_wood"
  } else if (color == "dark_wood") {
    addStyleSheets("/static/styles/color/white.css");
    this.setAttribute("data-color", "white");
    new_color = "white"
  };
  window.history.replaceState(null, null, window.location.pathname + "?f=" + new_color);
});
