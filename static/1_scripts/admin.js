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

function format_text(text) {
  text.innerHTML = text.innerHTML.replace(/&nbsp;/g, ' ');
  br = text.querySelectorAll("br");
  text.querySelectorAll("br");
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
  div = text.querySelectorAll("div");
  span = text.querySelectorAll("span");
  pre = text.querySelectorAll("pre");
  strong = text.querySelectorAll("strong");
  u = text.querySelectorAll("u");

  for (var i = 0; i < br.length; i++){
      br[i].removeAttribute("style"); br[i].removeAttribute("class")
  };
  for (var i = 0; i < pre.length; i++){
    Object.keys(pre[i].dataset).forEach(key=> {
    delete pre[i].dataset[key];
    })
  };

  for (var i = 0; i < img.length; i++){
      img[i].removeAttribute("style"); img[i].removeAttribute("class")
  };
  for (var i = 0; i < strong.length; i++){
      strong[i].removeAttribute("style"); strong[i].removeAttribute("class")
  };
  for (var i = 0; i < u.length; i++){
      u[i].removeAttribute("style"); u[i].removeAttribute("class")
  };
  for (var i = 0; i < p.length; i++){
      p[i].removeAttribute("style"); p[i].removeAttribute("class")
  };
  for (var i = 0; i < ul.length; i++){
      ul[i].removeAttribute("style"); ul[i].removeAttribute("class")
  };
  for (var i = 0; i < ol.length; i++){
      ol[i].removeAttribute("style"); ol[i].removeAttribute("class")
  };
  for (var i = 0; i < a.length; i++){
      a[i].removeAttribute("style"); a[i].removeAttribute("class")
  };
  for (var i = 0; i < span.length; i++){
      span[i].removeAttribute("style"); span[i].removeAttribute("class")
  };
  for (var i = 0; i < h1.length; i++){
      h1[i].removeAttribute("style"); h1[i].removeAttribute("class")
  };
  for (var i = 0; i < h2.length; i++){
      h2[i].removeAttribute("style"); h2[i].removeAttribute("class")
  };
  for (var i = 0; i < h3.length; i++){
      h3[i].removeAttribute("style"); h3[i].removeAttribute("class")
  };
  for (var i = 0; i < h4.length; i++){
      h4[i].removeAttribute("style"); h4[i].removeAttribute("class")
  };
  for (var i = 0; i < h5.length; i++){
      h5[i].removeAttribute("style"); h5[i].removeAttribute("class")
  };
  for (var i = 0; i < h6.length; i++){
      h6[i].removeAttribute("style"); h6[i].removeAttribute("class")
  };
  for (var i = 0; i < div.length; i++){
      div[i].removeAttribute("style"); div[i].removeAttribute("class")
      for (var i = 0; i < div.length; i++){
        Object.keys(div[i].dataset).forEach(key=> {
        delete div[i].dataset[key];
        })
      }
  };

  return text
};

on('body', 'input', '.smile_supported', function() {
    this.previousElementSibling.innerHTML = this.innerHTML.length
});

function get_and_change_btn(_this, url, hide) {
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url + _this.getAttribute("data-pk") + "/", true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    if (hide) {
      _this.innerHTML = "👁";
      _this.classList.add("publish_item");
      _this.classList.remove("hide_item");
    }
    else {
      _this.innerHTML = "🛇";
      _this.classList.remove("publish_item");
      _this.classList.add("hide_item");
    }
  }};
  link.send();
};

function send_category_data(form, url) {
  if (!form.querySelector(".form_title").value) {
    form.querySelector(".form_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  text_val1 = form.querySelector(".content_1");
  _val1 = format_text(text_val1);
  _text1 = _val1.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text1;
  form.append($input);
  form_data = new FormData(form);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url, true)
  }};
  link.send(form_data);
};

function send_content_data(url, field) {
  text_field = document.body.querySelector(".smile_supported");
  form = text_field.parentElement.parentElement;
  if (!text_field.innerHTML) {
    text_field.style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  _val1 = format_text(text_field);
  form_data = new FormData(form);
  form_data.append(field, _val1.innerHTML);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    console.log("ok");
  }};
  link.send(form_data);
};

on('body', 'click', '#edit_file_btn', function() {
  send_content_data("/edit_file/" + this.getAttribute("data-pk") + "/", "description");
});

function send_serve_data(form, url) {
  text_field = form.querySelector(".smile_supported");
  if (!text_field.innerHTML) {
    text_field.style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  _val1 = format_text(text_field);
  form_data = new FormData(form);
  form_data.append("description", _val1.innerHTML);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    console.log("ok");
  } else {
    console.log("not ok");
  }};
  link.send(form_data);
};

function send_post_data(form, url) {
  if (!form.querySelector(".form_title").value && !form.querySelector(".form_title").firstChild) {
    form.querySelector(".form_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    return
  }
  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url, true)
  }};
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
};
///////////SERVE //////////////////
on('body', 'click', '#create_serve_btn', function() {
  send_serve_data(this.parentElement, "/create_serve/");
});
on('body', 'click', '#create_tech_category_btn', function() {
  send_category_data(this.parentElement, "/create_tech_categories/");
});
on('body', 'click', '#create_serve_category_btn', function() {
  send_category_data(this.parentElement, "/create_serve_categories/");
});
on('body', 'click', '#edit_serve_btn', function() {
  send_serve_data(this.parentElement, "/edit_serve/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_object_content_btn', function() {
  send_content_data("/edit_content_item/" + this.getAttribute("data-pk") + "/", "content");
});
on('body', 'click', '#edit_serve_category_btn', function() {
  send_category_data(this.parentElement, "/edit_serve_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_tech_category_btn', function() {
  send_category_data(this.parentElement, "/edit_tech_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_serve', function() {
  delete_item("/delete_serve/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_serve_category', function() {
  delete_item("/delete_serve_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_tech_category', function() {
  delete_item("/delete_tech_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

/////////////////////////////
on('body', 'click', '#create_item_btn', function() {
  send_post_data(this.parentElement, "/create_item/");
});
on('body', 'click', '#create_category_btn', function() {
  send_category_data(this.parentElement, "/create_category/");
});
on('body', 'click', '#edit_item_btn', function() {
  send_post_data(this.parentElement, "/edit_item/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_category_btn', function() {
  send_category_data(this.parentElement, "/edit_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_item', function() {
  delete_item("/delete_item/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_category', function() {
  delete_item("/delete_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

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

on('body', 'change', '.load_tech_categories_from_level', function() {
  val = this.value;
  next = this.parentElement.nextElementSibling;
  block = next.querySelector(".form-control");
  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/load_serve_categories_from_level/" + this.value + "/", true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) {
          if ( link.status == 200 ) {
              block.innerHTML = link.responseText;
              if (block.getAttribute("data-cat-id")) {
                options = block.querySelectorAll("option");
                cat_id = block.getAttribute("data-cat-id");
                for (var i = 0; i < options.length; i++) {
                  if (options[i].val == cat_id) {
                    options[i].setAttribute("selected", "selected");
                  }
                }
              }
              next.classList.remove("hidden");
          }
      }
  };
  link.send( null );
});

on('body', 'change', '.load_serve_from_level', function() {
  val = this.value;
  next = this.parentElement.nextElementSibling;
  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/load_form_from_level/" + this.value + "/", true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) {
          if ( link.status == 200 ) {
              next.innerHTML = link.responseText;
              next.classList.remove("hidden");
          }
      }
  };
  link.send( null );
});

on('body', 'change', '.load_unical_object_form', function() {
  val = this.value;
  next = this.parentElement.nextElementSibling;
  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/unical_object_form/" + this.value + "/", true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) {
          if ( link.status == 200 ) {
              next.innerHTML = link.responseText;
          }
      }
  };
  link.send( null );
});

on('body', 'change', '.close_tech_categories', function() {
  options = this.querySelectorAll("option");
  next = this.parentElement.nextElementSibling;
  cats = next.querySelectorAll(".open_tech_category");

  for (var i = 0; i < options.length; i++) {
    if (options[i].selected) {
      cat = next.querySelector('[data-pk=' + '"' + options[i].value + '"' + ']');
      if (cat) {
        cat_options = cat.querySelectorAll("option");
        for (var i = 0; i < cat_options.length; i++) {
          cat_options[i].selected = false;
        }
      }
      cat.classList.add("hidden");
    }
    else {
      next.querySelector('[data-pk=' + '"' + options[i].value + '"' + ']').classList.remove("hidden");
    }
  }
});


on('body', 'click', '.hide_item', function() {
  get_and_change_btn(this, "/hide_item/", true);
});
on('body', 'click', '.publish_item', function() {
  get_and_change_btn(this, "/publish_item/", false);
});

on('body', 'click', '.show_user_history', function() {
  create_fullscreen("/load_user_history/" + this.getAttribute("data-pk") + "/", "item_fullscreen");
});

on('body', 'click', '.previous_click', function() {
  this.previousElementSibling.click();
});



on('body', 'change', '.add_photos_in_object', function() {
  form = this.parentElement;
  pk = form.getAttribute("data-pk");
  form_data = new FormData(form);
  data_block = document.body.querySelector(".doc_title");
  page_id = data_block.getAttribute("page-id");
  if (page_id == 43) {
    url = "/create_blog_images/";
  }
  else if (page_id == 63) {
    url = "/create_service_images/";
  }
  else if (page_id == 73) {
    url = "/create_store_images/";
  }
  else if (page_id == 83) {
    url = "/create_wiki_images/";
  }
  else if (page_id == 93) {
    url = "/create_work_images/";
  }

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(document.location.href, false)
  }};
  link.send(form_data);
});

on('body', 'change', '.add_file_in_object', function() {
  form = this.parentElement;
  item_pk = form.getAttribute("data-pk");
  item_types = form.getAttribute("item-type");
  data_types = form.getAttribute("data-type");
  form_data = new FormData(form);
  form_data.append("item_types", item_types);
  form_data.append("types", data_types);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/create_files/" + item_pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(document.location.href, false)
  }};
  link.send(form_data);
});

on('body', 'click', '.remove_file_from_object', function() {
  _this = this;
  pk = _this.getAttribute("data-pk");

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', "/delete_file/" + pk + "/", true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    _this.nextElementSibling.remove();
    _this.remove();
  }};
  link.send();
});
