// main scripts ver5
function get_document_opacity_0() {
  document.body.style.overflowY = "hidden";
  //document.body.style.marginRight = "20px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflowY = "scroll";
  //document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function elementInViewport(el){var bounds = el.getBoundingClientRect();return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));}

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  if (!container.innerHTML) {
    get_document_opacity_1();
    return
  };
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");

  try {
    meta_block = _window.querySelector(".doc_title");
    $link = meta_block.getAttribute("data-link");
    $title = meta_block.getAttribute("data-title");
    $object_id = meta_block.getAttribute("object-id");
    $page_id = meta_block.getAttribute("page-id");
    get_window_stat_meta($link, $title, $object_id, $page_id);
  } catch { null };

  _window.remove();

  if (!container.innerHTML) {
    get_document_opacity_1();
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.classList.remove("hide");
  };
  try {
    if (!container.querySelector(".order_window")) {
      document.body.querySelector(".price_section_block").style.display = "unset";
    }
  } catch { null };
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
function get_stat_form($link, $title, $object_id, $page_id) {
  formData = new FormData();
  formData.append('user_id', $user_id);
  formData.append('object_id', $object_id);
  formData.append('page_id', $page_id);
  formData.append('link', $link);
  formData.append('title', $title);
  formData.append('height', $height);
  formData.append('seconds', $seconds);
}

function get_stat_meta($link, $title, $object_id, $page_id) {
  //ip_block = document.body.querySelector(".ip_span");
  //if (
  //    document.body.querySelector("#is_superuser")
  //    || ip_block.innerHTML == "91.239.184.81"
  //    || ip_block.innerHTML == "176.59.23.228"
  //  ) {
  //  return
  //}
  //else
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

  if ($object_id) {
    analyticsData = {
      user_id: $user_id,
      object_id: $object_id,
      page_id: $page_id,
      link: $link,
      title: $title,
      height: $height,
      seconds: $seconds,
    }
  } else {
    analyticsData = {
      user_id: $user_id,
      object_id: 0,
      page_id: $page_id,
      link: $link,
      title: $title,
      height: $height,
      seconds: $seconds,
    }
  }

  fetch("/create_history/",
    {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      method: "POST",
      body: JSON.stringify(analyticsData)
    })
    .then(function(res){ console.log(res) })
    .catch(function(res){ console.log(res) })

  $height = parseFloat(window.innerHeight * 0.000264).toFixed(2);
  $seconds = 1;
  window.clearInterval(intervalListener);
}

function get_window_stat_meta($link, $title, $object_id, $page_id) {
  //return
  ip_block = document.body.querySelector(".ip_span");
  if (document.body.querySelector("#is_superuser")
      || ip_block.innerHTML == "91.239.184.81"
      || ip_block.innerHTML == "176.59.23.228") {
    return
  }
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

window.addEventListener("unload", function() {
  meta = document.body.querySelector(".doc_title");
  $title = meta.getAttribute("data-title");
  $page_id = meta.getAttribute("page-id");
  $object_id = meta.getAttribute("data-id") ? meta.getAttribute("data-id") : null;
  analyticsData = {
    user_id: $user_id,
    object_id: $object_id,
    page_id: $page_id,
    link: document.location.href,
    title: $title,
    height: $height,
    seconds: $seconds,
  }
  navigator.sendBeacon("/create_history/", JSON.stringify(analyticsData));
});

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
      stat_meta.querySelector(".device").innerHTML = "<span class='ip_span'>" +  data.ip + "</span> (" + _device + ") ";
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
        get_window_view_timer(120);
        $window_time_end = false;
      };

      // программа останавливает отчет времени просмотра элементов, на которых остановился
      // пользователь, записывает его всем новым элементам pag, затем их добавляет в основной
      // список стата, обнуляет счетчик и очищает список новых элементов.
      if ((window.innerHeight + window.pageYOffset) > offset) {
        offset = window.innerHeight + window.pageYOffset;
        $height = parseFloat(offset * 0.000264).toFixed(2);
      };

      //try {
          box = _block.querySelector('.next_page_list');
          if (box && box.classList.contains("next_page_list")) {
              inViewport = elementInViewport(box);
              if (inViewport) {
                  box.classList.remove("next_page_list");
                  paginate(box);
              }
          };
      //} catch {return}
    }
};
function paginate(block) {
        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
        link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link") + "&ajax=2", true);
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

function create_fullscreen(url, type_class, price) {
  container = document.body.querySelector("#fullscreens_container");

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        if (container.innerHTML) {
          prev_window = container.querySelector(".card_fullscreen");
          prev_window.classList.add("hide");
        };

        $parent_div = document.createElement("div");
        $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
        $parent_div.style.zIndex = 100 + count_items;
        $parent_div.style.opacity = "0";
        window_time_end = false;
        $window_height = 0;

        $hide_span = document.createElement("span");
        $hide_span.classList.add("this_fullscreen_hide");
        $loader = document.createElement("div");

        $loader.setAttribute("id", "fullscreen_loader");
        $hide_span.innerHTML = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
        $parent_div.append($hide_span);
        $parent_div.append($loader);
        container.prepend($parent_div);

          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (!price && height < 500 && !$loader.querySelector(".data_display")) {
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
            }
          };
          if (price) {
            create_order_form(price);
          }
      }
  };
  link.send();
};

function create_order_form(price) {
  console.log("create_order_form!");
  try {
    document.body.querySelector(".price_section_block").style.display = "none";
  } catch { null };

  fullscreens_container = document.body.querySelector("#fullscreens_container");
  serves_container = fullscreens_container.querySelector(".serves_container");

  banner_blocks = document.body.querySelectorAll(".banner_block");
  serves_container.parentElement.querySelector(".total_price").innerHTML = price;
  for (var i = 0; i < banner_blocks.length; i++) {
    if (banner_blocks[i].classList.contains("open_cat")) {
      banner_block_title = banner_blocks[i].querySelector(".section-title").innerHTML;

      tabs = banner_blocks[i].querySelector(".price_mode");
      tab_pk = tabs.querySelector(".active").getAttribute("data-pk");
      tab_panes = banner_blocks[i].querySelectorAll(".tab-pane");
      for (var k = 0; k < tab_panes.length; k++) {
        if (tab_panes[k].getAttribute("data-pk") == tab_pk) {
          serves_list = tab_panes[k].querySelectorAll(".hover");
          cat_title = tab_panes[k].querySelector(".get_serve_category_info").innerHTML;
          cat_id = tab_panes[k].querySelector(".get_serve_category_info").getAttribute("data-pk");
          serves = "";
          for (var i = 0; i < serves_list.length; i++) {
            serve = "<tr><td class='border-top' style='width:55%'><a class='get_serve_info pointer' data-pk='"
            + serves_list[i].querySelector(".get_serve_info").getAttribute("data-pk")
            + "'>"
            + serves_list[i].querySelector(".get_serve_info").innerHTML
            + "</a></td><td class='border-top price_td' style='width:15%'>"
            + serves_list[i].querySelector(".price_td").innerHTML
            + "</td><td class='border-top hours' style='width:15%'>"
            + serves_list[i].querySelector(".hours").innerHTML
            + "</td></tr>";

            serves += serve;
          }
        }
      }


      section = "<section class='banner_block mb-20 mt-20 border' style='width:100%;'><h4 class='section-title font-alt'>"
      + banner_block_title
      + "</h4><div><div><div class='tab-content tpl-minimal-tabs-cont section-text'><div class='tab-pane in active'><table class='table'><tbody><tr><th class='hidden-xs'><span data-pk='"
      + cat_id
      + "' class='pointer get_serve_category_info'>"
      + cat_title
      + "</span></th><th>Цена</th><th>Часы</th><th></th></tr>"

      + serves
      + "</tbody></table></div></div></div></section>";

      serves_container.innerHTML += section;

    }
  }
};

function change_this_fullscreen(_this, $loader) {
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

on('body', 'click', '.open_child_serves', function(event) {
  if (event.target.classList.contains("get_serve_info")) {
    return
  };
  parent_id = this.getAttribute("parent-pk");
  check = this.querySelector(".icon_parent");
  if (check.innerHTML == "▼") {
    check.innerHTML = "▲"
  }
  else {
    check.innerHTML = "▼"
  }
  childs = this.parentElement.querySelectorAll('[serve-pk=' + '"' + parent_id + '"' + ']');
  for (var i = 0; i < childs.length; i++) {
    if (childs[i].classList.contains("select_child_serve")) {
      childs[i].classList.toggle("hide");
    }
  }
});

on('body', 'click', '.select_child_serve', function(event) {
  _this = this;
  if (event.target.classList.contains("get_serve_info")) {
    return
  };
  if (_this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.classList.contains("order_window")) {
    counter = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.querySelector(".total_price");
  }
  else {
    counter = document.body.querySelector(".total_price_counter");
  }

  category_id = _this.parentElement.parentElement.parentElement.getAttribute("data-pk");
  categories = document.body.querySelector(".price_mode");
  category = categories.querySelector('[data-pk=' + '"' + category_id + '"' + ']');

  parent_id = _this.getAttribute("serve-pk");
  this_pk = _this.querySelector(".get_serve_info").getAttribute("data-pk");
  parent = _this.parentElement.querySelector('[parent-pk=' + '"' + parent_id + '"' + ']');
  serve_pk = parent.querySelector(".get_serve_info").getAttribute("data-pk");
  title = _this.querySelector(".get_serve_info").innerHTML;
  parent_title = parent.querySelector(".get_serve_info").innerHTML;
  price = _this.querySelector(".price").innerHTML;
  parent_price = parent.querySelector(".price").innerHTML;
  hours = _this.querySelector(".hours").innerHTML;
  parent_hours = parent.querySelector(".hours").innerHTML;

  if (parent_title.includes('Не выбрано')) {
    parent.classList.add("hover");
    parent.querySelector(".icon_check").innerHTML = "✔";
  }
  else if (title.includes('Не выбрано')){
    parent.classList.remove("hover");
    parent.querySelector(".icon_check").innerHTML = "";
  }

  if (!parent_price || parent_price == "-") {
    _parent_price = 0;
  }
  else {
    _parent_price = parent_price*1;
  }
  if (!price || price == "-") {
    _price = 0;
  }
  else {
    _price = price*1;
  }
  old_price = category.getAttribute("data-sum");
  category.setAttribute("data-sum", old_price - _parent_price + _price);

  counter.innerHTML = counter.innerHTML*1 - _parent_price + _price;

  parent.querySelector(".get_serve_info").innerHTML = title;

  if (_this.classList.contains("no_select_parent")) {
    counter.innerHTML = counter.innerHTML*1 - parent.querySelector(".price").innerHTML*1;
    parent.querySelector(".price").innerHTML = "";
    parent.querySelector(".hours").innerHTML = "";
  }
  else {
    if (parent.classList.contains("no_select_parent")) {
      parent.querySelector(".price").innerHTML = price;
      parent.querySelector(".hours").innerHTML = hours;
    }
    else {
      parent.querySelector(".price").innerHTML = price;
      parent.querySelector(".hours").innerHTML = hours;
    }
    parent.querySelector(".get_serve_info").setAttribute("data-pk", this_pk);
    _this.querySelector(".get_serve_info").setAttribute("data-pk", serve_pk);
  }

  _this.querySelector(".get_serve_info").innerHTML = parent_title;
  _this.querySelector(".price").innerHTML = parent_price;
  _this.querySelector(".hours").innerHTML = parent_hours;

  childs = _this.parentElement.querySelectorAll('[serve-pk=' + '"' + parent_id + '"' + ']');
  for (var i = 0; i < childs.length; i++) {
    if (childs[i].classList.contains("select_child_serve")) {
      childs[i].classList.toggle("hide");
    }
  }
  parent.querySelector(".icon_parent").innerHTML = "▼";
});

on('body', 'click', '.select_serve', function(event) {
  _this = this;
  if (event.target.classList.contains("get_serve_info")) {
    return
  };

  if (_this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.classList.contains("order_window")) {
    counter = _this.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.parentElement.querySelector(".total_price");
  }
  else {
    counter = document.body.querySelector(".total_price_counter");
  }
  serve_pk = _this.querySelector(".get_serve_info").getAttribute("data-pk");

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
          }
      };
    };
  };

  // найдем цену опции и сделаем цену числом
  serve_price = _this.querySelector(".price").innerHTML*1

  if (!_this.classList.contains("hover")){
    // если до нажатия опция не выбрана...
    counter.innerHTML = counter.innerHTML*1 + serve_price;
    _this.classList.add("hover");
    action_text = _this.querySelector(".action_text");
    action_text.innerHTML = '&nbsp;✔&nbsp;';
    action_text.setAttribute("tooltip", "Опция выбрана");
  }
  else {
    // если опция выбрана, надо снять выделение и счетчик уменьшить на сумму опции.
    // а также уменьшить "data-serve" счетчика
    counter.innerHTML = counter.innerHTML*1 - serve_price;
    _this.classList.remove("hover");
    action_text = _this.querySelector(".action_text");
    action_text.innerHTML = '&nbsp;+&nbsp;';
    action_text.setAttribute("tooltip", "Опция не выбрана");
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
    };
    nav = _this.parentElement.parentElement.parentElement;
    nav_items = nav.querySelectorAll(".yy");
    for (var i = 0; i < nav_items.length; i++) {
      nav_items[i].classList.remove("active", "in");
      nav_items[i].classList.add("pointer", "not_active")
    };
    _this.classList.add("active", "in");
    _this.classList.remove("pointer", "not_active");
    tabs = nav.querySelector(".tab-content");

    tabs_panes = tabs.querySelectorAll(".tab-pane");
    for (var i = 0; i < tabs_panes.length; i++) {
      if (is_price_mode) {
        serve_list = tabs_panes[i].querySelectorAll(".select_serve");
        for (var i2 = 0; i2 < serve_list.length; i2++){
            if (!serve_list[i2].classList.contains("is_default") && serve_list[i2].classList.contains("hover")){
              serve_list[i2].classList.remove("hover");
              old_price += serve_list[i2].querySelector(".price").innerHTML*1;

            }
        };
      };
      tabs_panes[i].classList.remove("active", "in")
    };

    cur = tabs.querySelector(tab_class);
    cur.classList.add("active", "in");

    if (is_price_mode) {
      counter.innerHTML = counter.innerHTML*1 - old_price + new_price;

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
  //if (this.getAttribute("href") == window.location.pathname){
  //  toast_info("Вы уже на этой странице");
  //  return
  //};
  ajax_get_reload(this.getAttribute("href"), true)
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

  background = getCookie("background");
  console.log(background);
  if (background != "") {
    color = background;
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
    addStyleSheets("/static/styles/color/old_paper.css");
    this.setAttribute("data-color", "old_paper");
    new_color = "old_paper"
  } else if (color == "old_paper") {
    addStyleSheets("/static/styles/color/dark_wood.css");
    this.setAttribute("data-color", "dark_wood");
    new_color = "dark_wood"
  } else if (color == "dark_wood") {
    addStyleSheets("/static/styles/color/white.css");
    this.setAttribute("data-color", "white");
    new_color = "white"
  }
  else {
    new_color = "white"
  };
  if (new_color != color) {
    setCookie("background", new_color, 90);
  }
});
on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

on('body', 'click', '.create_order_form', function() {
  create_fullscreen("/create_order/", "item_fullscreen", this.querySelector(".total_price_counter").innerHTML);
});
on('body', 'click', '.create_feedback_form', function() {
  create_fullscreen("/load_feedback/", "worker_fullscreen");
});

on('body', 'click', '.get_object_photo', function() {
  create_fullscreen("/image/" + this.getAttribute("data-pk") + "/", "photo_fullscreen");
});

on('body', 'click', '.get_tech_category_info', function() {
  create_fullscreen("/load_tech_category/" + this.getAttribute("data-pk") + "/", "worker_fullscreen");
});
on('body', 'click', '.get_serve_category_info', function() {
  create_fullscreen("/load_serve_category/" + this.getAttribute("data-pk") + "/", "worker_fullscreen");
});
on('body', 'click', '.get_serve_info', function() {
  create_fullscreen("/load_serve/" + this.getAttribute("data-pk") + "/", "worker_fullscreen");
});

on('body', 'click', '.next_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.body.querySelector("#fullscreen_loader"));
});
on('body', 'click', '.prev_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.body.querySelector("#fullscreen_loader"));
});

on('body', 'input', '.general_search', function() {
    _this = this;
    value = _this.value;

    if (value == "") {
      ajax_get_reload("/search/", true);
      return;
    }
    else if (value.length < 3) {
      return;
    }
    else if (_this.classList.contains("search-field") && !document.body.querySelector(".search_section")) {
      ajax_get_reload("/search/" + _this.value + "/", true);
      try{
        document.body.querySelector(".search_section").innerHTML.replaceAll(new RegExp(value, 'ig'), "<span class='selected'>" + value + "</span>");
      } catch { null };
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
          div.innerHTML = search.innerHTML.replaceAll(new RegExp(value, 'ig'), "<span class='selected'>" + value + "</span>");
          document.title = elem_.querySelector(".doc_title").getAttribute("data-title");
          window.history.replaceState(null, null, url);
        }
      }
      ajax_link.send();
  }
});

on('body', 'click', '.show_tech_category', function() {
  next_div = this.nextElementSibling;
  this.parentElement.classList.toggle("open_cat");
  counter = document.body.querySelector(".total_price_counter")
  if (next_div.classList.contains("hidden")) {
    counter.innerHTML = counter.innerHTML*1 + next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  } else {
    counter.innerHTML = counter.innerHTML*1 - next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  }
  this.querySelector(".cat_description").classList.toggle("hidden");
  this.querySelector(".cat_name").classList.toggle("hidden");
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


on('body', 'click', '#create_order_btn', function() {
  form = this.parentElement;
  if (!form.querySelector("#id_username").value) {
    form.querySelector("#id_username").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  else if (!form.querySelector("#id_email").value) {
    form.querySelector("#id_email").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }

  this.setAttribute("disable", "true");
  this.innerHTML = "Данные отправляются!";
  serves_input = "";
  serve_list = form.parentElement.querySelectorAll(".get_serve_info");
  for (var i = 0; i < serve_list.length; i++) {
    serves_input += serve_list[i].getAttribute("data-pk");
    if ((i + 1) != serve_list.length) {
      serves_input += ",";
    }
  }
  form_data = new FormData(form);

  data = document.body.querySelector(".object_data");
  object_id = data.getAttribute("data-pk");
  object_type = data.getAttribute("data-type");
  object_title = data.innerHTML;

  form_data.append("title", object_title);
  form_data.append("types", object_type);
  form_data.append("object_id", object_id);
  form_data.append("serve_list", serves_input);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/create_order/", true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    get_document_opacity_1();
    ajax_get_reload("/user_orders/", true);
  }};
  link.send(form_data);
});

on('body', 'click', '.remove_order', function() {
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/delete_order/" + this.getAttribute("data-pk") + "/", true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload("/user_orders/", true);
  }};
  link.send();
});

on('body', 'click', '.toggle_next_hide', function() {
  this.nextElementSibling.classList.toggle("hide");
});

on('body', 'click', '.edit_order', function() {
  create_fullscreen("/edit_order/" + this.getAttribute("data-pk") + "/", "item_fullscreen");
});

on('body', 'click', '#create_feedback_btn', function() {
  form = this.parentElement;
  if (!form.querySelector("#id_username").value) {
    form.querySelector("#id_username").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  else if (!form.querySelector("#id_email").value) {
    form.querySelector("#id_email").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  else if (!form.querySelector("#id_message").value) {
    form.querySelector("#id_message").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }

  this.setAttribute("disable", "true");
  this.innerHTML = "Данные отправляются!";
  form_data = new FormData(form);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/feedback/", true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    close_fullscreen();
    toast_info("Сообщение отправлено!")
  }};
  link.send(form_data);
});

on('body', 'change', '.load_tech_objects', function() {
  _this = this;
  block = _this.parentElement.querySelector(".loader_ul");
  if (_this.checked == false || block.querySelector("li")) {
    return
  };

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/load_tech_objects/" + _this.getAttribute("data-pk") + "/", true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
      elem_ = document.createElement('span');
      elem_.innerHTML = link.responseText;
      block.innerHTML = elem_.innerHTML;
  }};
  link.send();
});
