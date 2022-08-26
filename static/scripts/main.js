// main scripts ver5
function get_document_opacity_0() {
  document.body.style.overflowY = "hidden";
  document.body.style.marginRight = "20px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflowY = "scroll";
  document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  if (!container.innerHTML) {
    get_document_opacity_1();
    return
  };
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");

  meta_block = _window.querySelector(".doc_title");
  $link = meta_block.getAttribute("data-link");
  $title = meta_block.getAttribute("data-title");
  $object_id = meta_block.getAttribute("object-id");
  $page_id = meta_block.getAttribute("page-id");
  get_window_stat_meta($link, $title, $object_id, $page_id);

  _window.remove();

  if (!container.innerHTML) {
    get_document_opacity_1();
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "unset";
  };
};

$height = parseFloat(window.innerHeight * 0.000264).toFixed(2);
$seconds = 1;
$user_id = 0;
$page_time_end = false;

$window_height = 0;
$window_seconds = 1;
$window_time_end = false;

function get_window_view_timer(count) {
  // считаем время нахождения на странице, до 2х минут. При скролле перезапускаем.
  console.log("Время окна работает");
  i = 0;
  intervalListener2 = setInterval(() => {
    if (i < count) {
      $window_seconds += 1;
      console.log("window_seconds ", $window_seconds);
    }
    else {
      $window_time_end = true;
      window.clearInterval(intervalListener2);
    }
    i += 1;
  }, 1000);
};

function get_page_view_time(count) {
  // считаем время нахождения на странице, до count секунд. При скролле перезапускаем.
  console.log("Общее время страницы работает");
  i = 0;
  intervalListener = setInterval(() => {
    //console.log($seconds);
    if (i < count) {
      $seconds += 1;
    }
    else {
      $page_time_end = true;
      window.clearInterval(intervalListener);
    }
    i += 1;
  }, 1000);
};

function get_stat_meta($link, $title, $object_id, $page_id) {
  if (!$page_id) {
    return
  }
  // сначала активизируется функция отрисовки первого контента,
  // затем получается пользователь из куки,
  // потом мы получаем данные для отсылки статистики со всеми
  // примочками - таймеры и так далее.
  // при смене страницы повторяем только эту функцию
  console.log("======================");
  console.log("id пользователя",   $user_id);
  console.log("id объекта",        $object_id);
  console.log("id страницы",       $page_id);
  console.log("ссылка",            $link);
  console.log("название страницы", $title);
  console.log("накручено метров",  $height);
  console.log("затрачено секунд",  $seconds);
  console.log("======================");
  $height = parseFloat(window.innerHeight * 0.000264).toFixed(2);
  $seconds = 1;
  window.clearInterval(intervalListener);
}

function get_window_stat_meta($link, $title, $object_id, $page_id) {
  console.log("======================");
  console.log("id пользователя",   $user_id);
  console.log("id объекта",        $object_id);
  console.log("id страницы",       $page_id);
  console.log("ссылка",            $link);
  console.log("название страницы", $title);
  console.log("накручено метров",  $window_height);
  console.log("затрачено секунд",  $window_seconds);
  console.log("======================");
  $window_height = 0;
  $window_seconds = 1;
  window.clearInterval(intervalListener2);
}

///////////////
function get_or_create_cookie_user() {
  user = getCookie("user");
  var id;
  if (user != "") {
    id = user;
  }
  else {
    id = 0;
  }
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.overrideMimeType("application/json");
  ajax_link.open( 'GET', "/object_history/" + id + "/", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
      data = JSON.parse(ajax_link.responseText);
      console.log(data);
      if (data.device == 1) {
        _device = "Компьютер";
      }
      else {
        _device = "Телефон";
      }
      stat_meta = document.body.querySelector(".stat_meta");
      stat_meta.querySelector(".device").innerHTML = data.ip + " (" + _device + ") ";
      stat_meta.querySelector(".city").innerHTML = data.city_en + " (" + data.country_en + ") ";

      setCookie("user", data.id, 120);
      $user_id = data.id;
    }
  }
  ajax_link.send();
}

var delayedExec = function(after, fn) {
    var timer;
    return function() {
        timer && clearTimeout(timer);
        timer = setTimeout(fn, after);
    };
};

function scrolled(_block) {
    offset = 0;
    window.onscroll = function() {
      console.log("paginate");
      // программа отслеживает окончание прокрутки
      //scrollStopper();
      // программа считает секунды для внесения в стат страницы и списка, если он есть.
      if ($page_time_end) {
        console.log("перезапускаем счетчик страницы");
        get_page_view_time(120);
        $page_time_end = false;
      };
      if ($window_time_end) {
        console.log("перезапускаем счетчик окна");
        get_window_view_time(120);
        $window_time_end = false;
      };

      // программа останавливает отчет времени просмотра элементов, на которых остановился
      // пользователь, записывает его всем новым элементам pag, затем их добавляет в основной
      // список стата, обнуляет счетчик и очищает список новых элементов.
      if ((window.innerHeight + window.pageYOffset) > offset) {
        offset = window.innerHeight + window.pageYOffset;
        $height = parseFloat(offset * 0.000264).toFixed(2);
      };

      try {
          box = _block.querySelector('.next_page_list');
          if (box && box.classList.contains("next_page_list")) {
              inViewport = elementInViewport(box);
              if (inViewport) {
                  box.classList.remove("next_page_list");
                  paginate(box);
              }
          };
      } catch {return}
    }
};
function paginate(block) {
        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
        link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link"), true);
        link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

        link_3.onreadystatechange = function() {
            if (this.readyState == 4 && this.status == 200) {
                var elem = document.createElement('span');
                elem.innerHTML = link_3.responseText;
                block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML)
                block.remove()
            }
        }
        link_3.send();
};

function create_fullscreen(url, type_class) {
  container = document.body.querySelector("#fullscreens_container");

  if (container.innerHTML) {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "none";
  };

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  $parent_div = document.createElement("div");
  $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
  $parent_div.style.zIndex = 100 + count_items;
  $parent_div.style.opacity = "0";
  window_time_end = false;
  $window_height = 0;

  if (document.body.querySelector(".desctop_nav")) {
    hide_svg = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>'
  } else { hide_svg = "" };
  $hide_span = document.createElement("span");
  $hide_span.classList.add("this_fullscreen_hide");
  $loader = document.createElement("div");

  $loader.setAttribute("id", "fullscreen_loader");
  $hide_span.innerHTML = hide_svg;
  $parent_div.append($hide_span);
  $parent_div.append($loader);
  container.prepend($parent_div);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();
          get_window_view_timer(120);
          offset = 0;
          $window_seconds = 1;

          $loader.onscroll = function() {
            if ($window_time_end) {
              console.log("перезапускаем счетчик окна");
              get_window_view_timer(120);
              $window_time_end = false;
            };
            if ($loader.scrollHeight > offset) {
              offset = $loader.scrollHeight;
              $window_height = parseFloat(offset * 0.000264).toFixed(2);
            }
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          };
      }
  };
  link.send();
};

function change_this_fullscreen(_this, type_class) {
  _this.parentElement.classList.contains("col") ? $loader = _this.parentElement.parentElement.parentElement.parentElement : $loader = _this.parentElement.parentElement;
  $loader.innerHTML = "";
  $parent_div.style.opacity = "0";
  $parent_div.style.height = "35px";
  url = _this.getAttribute("href");

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          $parent_div = $loader.parentElement
          if (height < 500 && !$loader.querySelector(".data_display")){
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            $loader.style.overflowY = "unset";
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          };
          $window_seconds = 1;
          get_document_opacity_0();
          window_time_end = false;
          offset = 0;
          get_window_view_timer(120);

          $loader.onscroll = function() {
            if ($window_time_end) {
              console.log("перезапускаем счетчик");
              get_window_view_timer(120);
              $window_time_end = false;
            };
            if ($loader.scrollHeight > offset) {
                offset = $loader.scrollHeight;
                $window_height = parseFloat(offset * 0.000264).toFixed(2);
              }
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          }
      }
  };
  link.send();
};

//window.addEventListener("unload", function() {
//  navigator.sendBeacon("/analytics", JSON.stringify(analyticsData));
//});


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
        var body = document.querySelector('body');
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

on('body', 'click', '.select_serve', function(event) {
  _this = this;
  if (event.target.classList.contains("get_object_info")) {
    return
  };
  counter = document.body.querySelector(".total_price_counter");
  counter_serve_price = counter.getAttribute("data-serve")*1;
  counter_serve_list = counter.parentElement
    .getAttribute("data-servelist")
    .replace('[', "")
    .replace(']', "")
    .split(',');
  serve_pk = _this.querySelector(".get_object_info").getAttribute("data-pk");

  // для начала мы уберем выбранные опции во вкладках
  // выбранной категории (напр категории "моб. разработка")
  // а те, которые там по умолчанию выбраны, оставим.
  tab_panes = _this.parentElement.parentElement.parentElement.parentElement.querySelectorAll(".tab-pane");
  for (var i = 0; i < tab_panes.length; i++){
    // работаем только с теми таб панелями, которые не видны.
    if (!tab_panes[i].classList.contains("active")){
      serve_list = tab_panes[i].querySelectorAll(".select_serve");
      for (var i2 = 0; i2 < serve_list.length; i2++){
        // также нужно уменьшить счетчик цены на сумму всех выбранных опций в других
        // вкладках. А также уменьшить "data-serve" счетчика
          if (!serve_list[i2].classList.contains("is_default") && serve_list[i2].classList.contains("hover")){
            serve_list[i2].classList.remove("hover");
            _serve_price = serve_list[i2].querySelector(".price").innerHTML*1
            counter.innerHTML = counter.innerHTML*1 - _serve_price;
            index = counter_serve_list.indexOf(serve_pk);
            if (index > -1) {
              counter_serve_list.splice(index, 1);
            }
          }
      };
    };
  };

  counter.parentElement.setAttribute("data-servelist", counter_serve_list)
  // найдем цену опции и сделаем цену числом
  serve_price = _this.querySelector(".price").innerHTML*1

  if (!_this.classList.contains("hover")){
    // если до нажатия опция не выбрана...
    counter.innerHTML = counter.innerHTML*1 + serve_price;
    _this.classList.add("hover");
    _this.querySelector(".action_text").innerHTML = 'Выбрано';
    counter_serve_list.push(serve_pk);
    counter.parentElement.setAttribute("data-servelist", counter_serve_list)
  }
  else {
    // если опция выбрана, надо снять выделение и счетчик уменьшить на сумму опции.
    // а также уменьшить "data-serve" счетчика
    counter.innerHTML = counter.innerHTML*1 - serve_price;
    _this.classList.remove("hover");
    _this.querySelector(".action_text").innerHTML = 'Выбрать';
    var index = counter_serve_list.indexOf(serve_pk);
    if (index > -1) {
      counter_serve_list.splice(index, 1);
      counter.parentElement.setAttribute("data-servelist", counter_serve_list)
    }
  }
});

function service_tab_action(_this, tab_class) {
  is_price_mode = false;
  if (_this.parentElement.classList.contains("price_mode")) {
      is_price_mode = true;
  }
  counter = document.body.querySelector(".total_price_counter");
  if (!_this.classList.contains("active")){
    if (is_price_mode) {
      old_price = _this.parentElement.querySelector(".active").getAttribute("data-sum")*1;
      new_price = _this.getAttribute("data-sum")*1;
      serves_ids = counter.parentElement.getAttribute("data-servelist").slice(-1).split(",");
      console.log(serves_ids);
    };
    nav = _this.parentElement.parentElement.parentElement;
    nav_items = nav.querySelectorAll(".yy");
    for (var i = 0; i < nav_items.length; i++) {
      nav_items[i].classList.remove("active", "in");
      nav_items[i].classList.add("pointer")
    };
    _this.classList.add("active", "in");
    _this.classList.remove("pointer");
    tabs = nav.querySelector(".tab-content");

    tabs_panes = tabs.querySelectorAll(".tab-pane");
    for (var i = 0; i < tabs_panes.length; i++) {
      if (is_price_mode) {
        serve_list = tabs_panes[i].querySelectorAll(".select_serve");
        for (var i2 = 0; i2 < serve_list.length; i2++){
            if (!serve_list[i2].classList.contains("is_default") && serve_list[i2].classList.contains("hover")){
              serve_list[i2].classList.remove("hover");
              old_price += serve_list[i2].querySelector(".price").innerHTML*1;
              index = serves_ids.indexOf(serve_list[i2].querySelector(".get_object_info").getAttribute("data-pk"));
              if (index >= 0) {
                serves_ids.splice(index, 1);
              }
            }
        };
      };
      tabs_panes[i].classList.remove("active", "in")
    };

    cur = tabs.querySelector(tab_class);
    cur.classList.add("active", "in");

    if (is_price_mode) {
    counter.innerHTML = counter.innerHTML*1 - old_price + new_price;

    // после смены таба перезапишем список выбранных опций
    new_serve_list = cur.querySelectorAll(".is_default");
    for (var i = 0; i < new_serve_list.length; i++) {
      serves_ids.push(new_serve_list[i].querySelector(".get_object_info").getAttribute("data-pk"))
    };
    counter.parentElement.setAttribute("data-servelist", serves_ids);
    ///////
    };
  }
};

function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});}

//window.addEventListener("unload", function() {
//  analyticsData = data;
//  navigator.sendBeacon("/analytics", JSON.stringify(analyticsData));
//});
on('body', 'click', '.ajax', function(event) {
  event.preventDefault();
  if (this.getAttribute("href") == window.location.pathname){
    toast_info("Вы уже на этой странице");
    return
  };
  ajax_get_reload(this.getAttribute("href"))
});


on('body', 'click', '.s_1', function() {
  service_tab_action(this, ".tab_1")
});
on('body', 'click', '.s_2', function() {
  service_tab_action(this, ".tab_2")
});
on('body', 'click', '.s_3', function() {
  service_tab_action(this, ".tab_3")
});
on('body', 'click', '.s_4', function() {
  service_tab_action(this, ".tab_4")
});
on('body', 'click', '.s_5', function() {
  service_tab_action(this, ".tab_5")
});
on('body', 'click', '.s_6', function() {
  service_tab_action(this, ".tab_6")
});
on('body', 'click', '.s_7', function() {
  service_tab_action(this, ".tab_7")
});

on('body', 'click', '.anon_color_change', function() {
  color = "white";
  backgroud = getCookie("backgroud");
  if (backgroud != "") {
    color = backgroud;
  }
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
  if (new_color != color) {
    setCookie("backgroud", new_color, 90);
  }
});
on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

on('body', 'click', '.get_object_info', function() {
  create_fullscreen("/load_item/?_object_type=" + this.getAttribute("data-type") + "&_owner_type=" + this.getAttribute("owner-type") + "&_object_pk=" + this.getAttribute("data-pk") + "&_owner_pk=" + this.getAttribute("owner-pk"), "worker_fullscreen");
});

on('body', 'click', '.next_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});
on('body', 'click', '.prev_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});

on('body', 'input', '.general_search', function() {
    _this = this;
    value = _this.value;

    if (value == "") {
      ajax_get_reload("/search/");
      return;
    }
    else if (_this.classList.contains("search-field") && !document.body.querySelector(".search_section")) {
      ajax_get_reload("/search/" + _this.value + "/");
      return;
    }
    else if (document.body.querySelector(".search_section")) {
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
          document.body.querySelector(".search_page").value = _this.value;
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          search = elem_.querySelector(".search_section");
          div = document.body.querySelector(".search_section");
          div.innerHTML = '';
          console.log("search_block", search);
          console.log("target_block", div);
          div.innerHTML = search.innerHTML;
          document.title = elem_.querySelector(".doc_title").getAttribute("data-title");
          window.history.replaceState(null, null, url);
        }
      }
      ajax_link.send();
  }
});

on('body', 'click', '.show_tech_category', function() {
  next_div = this.nextElementSibling;
  counter = document.body.querySelector(".total_price_counter")
  if (next_div.classList.contains("hidden")) {
    counter.innerHTML = counter.innerHTML*1 + next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  } else {
    counter.innerHTML = counter.innerHTML*1 - next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  }
  this.querySelector(".cat_description").classList.toggle("hidden");
  this.querySelector(".cat_name").classList.toggle("hidden")
  next_div.classList.toggle("hidden")
});

on('body', 'click', '#logg', function() {
  _this = this;
  form = _this.parentElement;
  response = form.querySelector(".api_response");

  if (!form.querySelector("#id_username").value){
    form.querySelector("#id_username").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите логин!";
    response.classList.add("error");
    return
  }
  else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите пароль!";
    response.classList.add("error")
    return
  }
  else {
    _this.disabled = true;
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/login/", true );
  //link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    window.location.href = "/"
    }
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "Логин или пароль - неверный!";
    response.classList.add("error");
    form.querySelector("#id_username").style.display = "block";
    form.querySelector("#id_username").value = '';
    form.querySelector("#id_password").value = '';
  }};
  link.send(form_data);
});

on('body', 'click', '#signup', function() {
  _this = this;
  form = _this.parentElement;
  username = form.querySelector("#id_username");
  response = form.querySelector(".api_response");
  if (!username.value){
    username.style.border = "1px #FF0000 solid";
    toast_error("Логин - обязательное поле!");
    return
  } else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    toast_error("Пароль - обязательное поле!");
    return
  }
  else {
    this.disabled = true
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/signup/", true );

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    window.location.href = "/"
    }
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "not ok";
    response.classList.add("error");
  }};
  link.send(form_data);
});

on('body', 'click', '.show_next_element', function() {
  this.nextElementSibling.classList.toggle("hidden")
});
