function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

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

function ajax_get_reload(url) {
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=1", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        sidebar = elem_.querySelector(".sidebar");
        rtr = document.getElementById('ajax');
        rtr.innerHTML = elem_.innerHTML;
        window.scrollTo(0,0);
        window.history.replaceState(null, null, url);
      }
    }
    ajax_link.send();
};
