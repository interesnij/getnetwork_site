(function ($) {


    "use strict";
    preloader();


    loadElementorInit();
    changeStyle();

    async function reloadAjax($off, $el = $(document)) {

        dsnGrid.initTheme();

        $('.hidden[data-dsn="bg_section"]').each(function () {
            $(this).next().append($(this));
            $(this).removeClass('hidden');
        });


        if (!$off) {
            window.$effectScroll = await effectScroller();
            window.$animate = await dsnGrid.effectAnimate();
        }


        await navMenu().init();
        await dsnGrid.removeWhiteSpace(".site-header ul.extend-container li > a");

        await imgPosition($el);
        await $effectScroll.start();
        await $animate.allInt();
        await magnificPopup($el);
        await projectSlider().run($el);
        await justifiedGallery($el);
        await slider($el).run();
        await Isotope($el);
        await loadMore($el);
        await accordion($el);
        await initMap($el);
        await linkRightPaginate($el);
        await mouseCirMove($off);


        $el.find("a.vid").YouTubePopUp();


        await dsnGrid.dsnAjax({
           success: () => {
                reloadAjax(true).catch($err => {
                    console.error($err);
                });
                ScrollTrigger.refresh();
            },
            onComplete(){
                ScrollTrigger.refresh();
            }
        }).ajaxLoad();

        await effctStickyNavBar();
        await dropHash($off);
        await list_project();
        await inputNumber();


    }


    function reloadElment($off, $el = $(document)) {


        if (typeof $animate !== 'undefined') {
            $animate.parallaxImgHover($el);
            $animate.parallaxImg($el);
            $animate.moveSection($el);
            $animate.animations($el);
            $animate.translateSection($el);

        }

        imgPosition($el);
        magnificPopup($el);
        $el.find("a.vid").YouTubePopUp();
        projectSlider().run($el);
        justifiedGallery($el)
        Isotope($el);

        accordion($el);
        initMap($el);
        slider($el).run();
        loadMore($el)


    }


    /**
     *
     * @returns {Object}
     */
    function navMenu() {
        const siteHeader = $(".site-header");
        return {
            init: function () {
                if (!siteHeader.length) return;
                let $parent = this;
                this.cutterText();
                $parent.hamburgerOpen();
            },
            cutterText: function () {
                let text_menu = siteHeader.find(".menu-icon .text-menu");
                if (text_menu.length <= 0) return;
                let text_button = text_menu.find(".text-button");
                let text_open = text_menu.find(".text-open");
                let text_close = text_menu.find(".text-close");

                dsnGrid.convertTextLine(text_button);
                dsnGrid.convertTextLine(text_open);
                dsnGrid.convertTextLine(text_close);
                text_close = null;
                text_open = null;
                text_button = null;
                text_menu = null;
            },
            hamburgerOpen: function () {
                const mainIcon = siteHeader.find(".menu-icon");
                const mainNav = siteHeader.find(".main-navigation");


                let tl = gsap.timeline({
                    paused: true, onReverseComplete: function () {
                        setTimeout(function () {
                            mainIcon.find(".icon-top , .icon-bottom").css("transform", "").css("display", "");
                        }, 50);

                        console.log('onReverseComplete : tl');


                    },
                });


                var menuClick = gsap.timeline({
                    onReverseComplete: function () {
                        menuClick = gsap.timeline();
                        console.log('onReverseComplete : menuClick');

                    }
                });
                let Ease = Power3.easeOut;

                //--> Open Menu

                tl.set(mainIcon.find(".icon-center"), {display: "none"});
                tl.to(mainIcon.find(".icon-top"), 0.5, {width: 23, rotation: 45, top: 0, y: 6, ease: Ease});
                tl.to(mainIcon.find(".icon-bottom"), 0.5, {
                    width: 23,
                    rotation: -45,
                    y: -6,
                    top: 0,
                    ease: Ease,
                }, 0);
                tl.call(function () {
                    mainIcon.toggleClass('nav-active');
                }, 0);

                tl.to(mainNav, 0.5, {y: "0%", autoAlpha: 1, ease: Ease}, 0);
                tl.fromTo(mainNav, 0.5, {y: "-100%", autoAlpha: 0}, {
                    y: "0%",
                    autoAlpha: 1,
                    ease: Expo.easeInOut,
                }, 0);

                tl.staggerTo(mainNav.find("ul.extend-container > li > a .dsn-title-menu"), 0.5, {
                    autoAlpha: 1,
                    y: 0,
                    ease: Back.easeOut.config(1.7),
                }, 0.05);


                tl.set(mainNav.find("ul.extend-container > li > a .dsn-meta-menu"), {
                    autoAlpha: 1,
                    ease: Ease,
                });
                tl.to(mainNav.find(".container-content"), 1, {autoAlpha: 1}, "-=1");
                tl.reverse();


                mainNav.find("ul.extend-container > li.dsn-drop-down").on("click", function (e) {
                    e.stopPropagation();

                    if (menuClick._tDur > 0) return;
                    menuClick = gsap.timeline({
                        onReverseComplete: function () {
                            menuClick = gsap.timeline();
                        },
                    });
                    menuClick.set($(this).find("ul"), {display: "flex"});
                    menuClick.to(mainNav.find("ul.extend-container > li > a ").find(".dsn-title-menu , .dsn-meta-menu"), 0.5,
                        {y: -30, autoAlpha: 0, ease: Back.easeIn.config(1.7)});
                    menuClick.set(".site-header .extend-container .main-navigation ul.extend-container li", {overflow: "hidden"});
                    menuClick.staggerFromTo($(this).find("ul li"), 0.5, {x: 50, autoAlpha: 0}, {
                        x: 0,
                        autoAlpha: 1,
                        ease: Back.easeOut.config(1.7),
                    }, 0.1);


                });

                mainIcon.off("click");
                mainIcon.on("click", function () {
                    console.log('mainIcon:click');

                    if (!tl.isActive()) {
                        menuClick.reverse(-1);
                        tl.reversed(!tl.reversed());
                        menuClick = gsap.timeline();
                    }


                });
                let backMenu = $(".dsn-back-menu");
                backMenu.off("click");
                backMenu.on("click", function (e) {
                    console.log('backMenu:click');
                    e.stopPropagation();
                    menuClick.reverse();

                });

            },
        };


    }


    function Isotope($el = $(document)) {

        $el.find(".dsn-isotope").each(($key, $item) => {
            setTimeout(function () {
                $build.iso[$($item).data('dsn-iso')] = $($item).isotope({
                    itemSelector: dsnGrid.getData($item, 'item', '.grid-item'),
                    horizontalOrder: true,
                    fitWidth: false,
                });
            }, 500)

        })


        $el.find('.root-posts').each(function () {
            const $button = $(this).find('.dsn-filtering .filtering button'),
                $item = $(this).find('.dsn-posts');


            if (!$button.length || !$item.length) return;

            $build.iso[$($item).data('dsn-iso')] = $item.isotope({
                itemSelector: '.grid-item'
            });

            $button.off("click");
            $button.on("click", function () {
                $(this).addClass('active').siblings().removeClass("active");
                $item.isotope({
                    filter: $(this).attr("data-filter"),
                });
            });


        });

    }

    function loadElementorInit() {
        $wind.on("elementor/frontend/init", function () {
            if (typeof elementor !== "undefined") {
                elementorFrontend.hooks.addAction("frontend/element_ready/global", function ($elemnt) {
                    let hdev = $elemnt.find('.hdev-element-wrap');
                    let next_el = hdev.next();
                    if (hdev.hasClass('has-animate-text')) {
                        next_el.attr('data-dsn-animate', 'section');
                    }
                    reloadElment(true, $elemnt);
                });

            }
        });
    }


    /**
     * t is using translate3d to perform a momentum based scrolling
     * (aka inertial scrolling) on modern browsers.
     *
     */
    function effectScroller() {
        const Scrollbar = window.Scrollbar;
        const locked_scroll = "locked-scroll";
        const contentScrollbar = "dsn-scrollbar";


        return {
            /**
             *
             * @param $print
             * @returns {boolean}
             * Check smooth scroll is enable or not
             */
            isScroller: function ($print) {

                const myScrollbar = document.getElementById(contentScrollbar);


                let hasSc = !$body.hasClass("dsn-effect-scroll") || dsnGrid.isMobile() || myScrollbar === null;
                if (hasSc && $print) {
                    $body.addClass("dsn-mobile");
                }
                return !hasSc;
            },

            contactForm: function () {
                const form = $('.contact-modal .contact-container');
                if (form.length)
                    Scrollbar.init(form.get(0), {
                        damping: dsnParam.scrollbar.speed,
                    });
            },

            woocommerceList: function () {
                const form = $('#dsn_cart .woocommerce ul.cart_list, #dsn_cart .woocommerce ul.product_list_widget');
                if (form.length)
                    Scrollbar.init(form.get(0), {
                        damping: dsnParam.scrollbar.speed,
                    });
            },
            sidebarScroll: function () {
                const comment = document.querySelector(".dsn-sidebar .sidebar-single");

                if (comment !== null)
                    Scrollbar.init(comment, {
                        damping: dsnParam.scrollbar.speed || 0.05,
                    });
            },


            /**
             *  Init Smooth ScrollBar
             */
            start: function () {
                $body.removeClass(locked_scroll);
                this.contactForm();
                if (!this.isScroller(true)) return;


                class EdgeEasingPlugin extends Scrollbar.ScrollbarPlugin {
                    constructor() {
                        super(...arguments);
                        this._remainMomentum = {
                            x: 0,
                            y: 0
                        };
                    }

                    transformDelta(delta) {
                        const {
                            limit,
                            offset
                        } = this.scrollbar;
                        const x = this._remainMomentum.x + delta.x;
                        const y = this._remainMomentum.y + delta.y; // clamps momentum within [-offset, limit - offset]

                        this.scrollbar.setMomentum(Math.max(-offset.x, Math.min(x, limit.x - offset.x)), Math.max(-offset.y, Math.min(y, limit.y - offset.y)));
                        return {
                            x: 0,
                            y: 0
                        };
                    }

                    onRender(remainMomentum) {
                        Object.assign(this._remainMomentum, remainMomentum);
                    }

                }

                EdgeEasingPlugin.pluginName = "edgeEasing";
                Scrollbar.use(EdgeEasingPlugin);
                const scroll = Scrollbar.init(document.getElementById(contentScrollbar), {
                    damping: dsnParam.scrollbar.speed,
                    renderByPixel: true,
                    continuousScrolling: true,
                    syncCallbacks: true,
                    thumbMinSize: 20
                });


                this.sidebarScroll();
                this.woocommerceList();


                // ScrollTrigger.defaults({
                //     scroller: "#dsn-scrollbar"
                // });
                // $effectScroll.getListener(function (status) {
                //     ScrollTrigger.refresh();
                // });


                ScrollTrigger.defaults({
                    scroller: "#dsn-scrollbar",
                    pinType: 'transform'
                });
                ScrollTrigger.scrollerProxy(document.getElementById(contentScrollbar), {
                    scrollTop(value) {
                        if (arguments.length) {
                            scroll.scrollTop = value; // setter
                        }

                        return scroll.scrollTop; // getter
                    },

                    scrollLeft(value) {
                        if (arguments.length) {
                            scroll.scrollLeft = value; // setter
                        }

                        return scroll.scrollLeft; // getter
                    }

                });
                scroll.addListener(() => {
                    ScrollTrigger.update();
                });

            },


            /**
             *  locked smooth scrollbar
             */
            locked: function () {
                $body.addClass(locked_scroll);
                if (this.isScroller()) {
                    let scroll = this.getScrollbar();
                    if (scroll !== undefined) {
                        scroll.destroy();
                    }
                    scroll = null;
                }
            },

            /**
             *
             * @param $id
             * @returns {*}
             * Gets scrollbar on the given element. If no scrollbar instance exsits, returns undefined:
             */
            getScrollbar: function ($id) {
                if ($id === undefined) {
                    return Scrollbar.get(document.getElementById(contentScrollbar));
                }
                return Scrollbar.get(document.querySelector($id));
            },

            /**
             *
             * @param $obj
             * @param $useWin
             *
             * Since scrollbars will not fire a native scroll event
             */
            getListener: function ($obj, $useWin = true) {
                if ($obj === undefined) return;
                let $this = this;
                if ($this.isScroller()) {
                    $this.getScrollbar().addListener($obj);
                } else {
                    if ($useWin)
                        $wind.on("scroll", $obj);
                }
                $this = null;
            },

            /**
             *  set scroll bar position
             */
            scrollNavigate: function () {
                let top = $(".wrapper").offset();
                top = top ? top.top : 0;
                $(".scroll-top , .scroll-to-top").on("click", function () {
                    dsnGrid.scrollTop(0, 2);
                });

                $(".scroll-d").on("click", function () {
                    dsnGrid.scrollTop(top, 2,
                        ($(".scrollmagic-pin-spacer").height() * -1) - 200 || -200);
                });

            },
        };

    }

    function imgPosition($el) {
        $el.find("[data-dsn-position]").each(function () {

            if (this.nodeName === "IMG")
                $(this).css("object-position", dsnGrid.getData(this, "position", "center"));
            else
                $(this).css("background-position", dsnGrid.getData(this, "position", "center"));
        });
    }

    function loadMore($el = $(document)) {
        dsnGrid.loadMore($el, {
            success: function () {
                $animate.parallaxImgHover();
            }
        });
    }

    /**
     * Popup Image
     */
    function magnificPopup($el = $(document)) {
        let option = {
            delegate: "div.effect-popup",
            type: "image",
            closeOnContentClick: false,
            closeBtnInside: false,
            mainClass: "mfp-with-zoom", // this class is for CSS animation below
            image: {
                verticalFit: true,
                titleSrc: function (item) {
                    return item.el.attr("data-title");
                },
            },
            gallery: {
                enabled: true,
            },
            zoom: {
                enabled: true,
                duration: 300, // don't foget to change the duration also in CSS
                // easing: "cubic-bezier(0.36, 0, 0.66, -0.56)", // CSS transition easing function
                opener: function (element) {
                    return element.find("img");
                },

            },
            callbacks: {
                open: function () {
                    $("html").css({margin: 0});
                }
            },

        };
        $el.find(".gallery-portfolio").each(function () {
            $(this).magnificPopup(option);
        });

        if ($el.find(".has-popup .pop-up").length) option.delegate = "div.pop-up";
        if ($el.find(".has-popup .img-box-parallax").length) option.delegate = "div.img-box-parallax";
        $el.find(".has-popup").magnificPopup(option);
        option.delegate = "a";
        $el.find(".woocommerce-product-gallery__image").magnificPopup(option);

    }


    /**
     *
     * @returns {{swiper: swiper, dsnProgressIndicator: ((function(*, *): (*))|*), run: run}}
     */
    function projectSlider() {


        return {

            swiper: function ($id, $obj) {

                $id = dsnGrid.convertToJQuery($id);
                
                $obj = $.extend(true, {
                    speed: 1000,
                    pagination: {
                        el: $id.find(".swiper-pagination").get(0),
                        clickable: true,
                        type: dsnGrid.getData($id.find(".swiper-pagination"), 'type', 'bullets')
                    },
                    navigation: {
                        nextEl: $id.find(".swiper-next ,.next-container").get(0),
                        prevEl: $id.find(".swiper-prev , .prev-container").get(0),
                    }
                }, $obj);


                $obj.breakpoints['0'] = {
                    slidesPerView: 1,
                    spaceBetween: 0,
                }


                $obj = this.dsnProgressIndicator($id, $obj);


                const $s = new Swiper($id.find(".swiper-container").get(0), $obj);
                dsnGrid.addSwiper($s);


            },

            dsnProgressIndicator: function ($id, $obj) {

                if (!$id.find('.dsn-progress-indicator').length) return $obj;

                $obj.pagination = {
                    el: $id.find(".dsn-controls .dsn-numbers span:not(.full-number)").get(0),
                    type: "custom",
                    clickable: true,
                    renderCustom: function (swiper, current, total) {
                        $id.find(".dsn-controls .dsn-numbers span.full-number").html(dsnGrid.numberText(total));
                        gsap.to($id.find(".dsn-controls .dsn-progress .dsn-progress-indicator"), 1, {width: (current / total) * 100 + "%"});
                        return dsnGrid.numberText(current);
                    },
                }

                return $obj;
            },

            run: function ($el = $(document)) {
                const $this = this;
                $el.find(".dsn-swiper").each(function () {

                    let option = dsnGrid.getData(this, "option", {});
                    let syn = $(this).parent().find(dsnGrid.getData(this, "controllers"));
                    if (syn.length) {
                        option['thumbs'] = {
                            swiper: {
                                el: syn.find('.swiper-container').get(0),
                                allowTouchMove: false,
                                slidesPerView: 1,
                                speed: option.speed || 1000,
                                parallax: true,
                                autoHeight: true
                            }
                        };

                    }
                    if (syn.length) {
                        option['thumbs'] = {
                            swiper: {
                                el: syn.find('.swiper-container').get(0),
                                allowTouchMove: false,
                                slidesPerView: 1,
                                speed: option.speed || 1000,
                                parallax: true,
                                autoHeight: true
                            }
                        };

                        option.breakpoints['768'] = {
                            slidesPerView: 1,
                            spaceBetween: 0,
                        }

                    }
                    $this.swiper(this, option);
                });

            },
        };

    }


    /**
     *
     *  - Create an high quality justified gallery
     *    of image
     *
     */
    function justifiedGallery($el = $(document)) {

        $el.find(".gallery-portfolio").each(function () {
            $(this).justifiedGallery(dsnGrid.getData(this, 'option', {}));
        });

    }


    function accordion($el = $(document)) {


        $el.find(".dsn-accordion").each(function () {
            const $this = $(this),
                acc_q = $this.find(".accordion__question");
            acc_q.on("click", function () {
                const content = $(this).next();
                $this.find(".accordion__answer").not(content).slideUp(400);
                acc_q.not(this).removeClass("expanded");
                $(this).toggleClass("expanded");

                $this.find('.accordion__item').removeClass("active");
                if ($(this).hasClass('expanded'))
                    $(this).parents('.accordion__item').addClass("active");

                content.slideToggle(400);

            });
        });

    }


    function initMap($el = $(document)) {


        const map_id = $el.find(".map-custom"),
            map_scropt_id = document.getElementById("map_api");


        if (map_id.length <= 0) return;
        // Styles a map in night mode.

        if (map_scropt_id === null) {
            var GOOGLE_MAP_KEY = dsnParam.map.api;

            var script = document.createElement("script");
            script.type = "text/javascript";
            script.id = "map_api";
            script.src = "https://maps.googleapis.com/maps/api/js?key=" + GOOGLE_MAP_KEY;
            document.body.appendChild(script);

        }


        setTimeout(function () {
            try {
                map_id.each(function () {
                    let map_att = $(this),
                        lat = dsnGrid.getData(this, "lat"),
                        leg = dsnGrid.getData(this, "len"),
                        zoom = dsnGrid.getData(this, "zoom");
                    var letLeng = new google.maps.LatLng(lat, leg),
                        map = new google.maps.Map(map_att.get(0), {
                            center: {
                                lat: lat,
                                lng: leg,
                            },
                            mapTypeControl: false,
                            scrollwheel: false,
                            draggable: true,
                            streetViewControl: false,
                            navigationControl: false,
                            zoom: zoom,
                            styles: [
                                {
                                    "featureType": "water",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#e9e9e9",
                                        },
                                        {
                                            "lightness": 17,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "landscape",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#f5f5f5",
                                        },
                                        {
                                            "lightness": 20,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "road.highway",
                                    "elementType": "geometry.fill",
                                    "stylers": [
                                        {
                                            "color": "#ffffff",
                                        },
                                        {
                                            "lightness": 17,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "road.highway",
                                    "elementType": "geometry.stroke",
                                    "stylers": [
                                        {
                                            "color": "#ffffff",
                                        },
                                        {
                                            "lightness": 29,
                                        },
                                        {
                                            "weight": 0.2,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "road.arterial",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#ffffff",
                                        },
                                        {
                                            "lightness": 18,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "road.local",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#ffffff",
                                        },
                                        {
                                            "lightness": 16,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "poi",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#f5f5f5",
                                        },
                                        {
                                            "lightness": 21,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "poi.park",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#dedede",
                                        },
                                        {
                                            "lightness": 21,
                                        },
                                    ],
                                },
                                {
                                    "elementType": "labels.text.stroke",
                                    "stylers": [
                                        {
                                            "visibility": "on",
                                        },
                                        {
                                            "color": "#ffffff",
                                        },
                                        {
                                            "lightness": 16,
                                        },
                                    ],
                                },
                                {
                                    "elementType": "labels.text.fill",
                                    "stylers": [
                                        {
                                            "saturation": 36,
                                        },
                                        {
                                            "color": "#333333",
                                        },
                                        {
                                            "lightness": 40,
                                        },
                                    ],
                                },
                                {
                                    "elementType": "labels.icon",
                                    "stylers": [
                                        {
                                            "visibility": "off",
                                        },
                                    ],
                                },
                                {
                                    "featureType": "transit",
                                    "elementType": "geometry",
                                    "stylers": [
                                        {
                                            "color": "#f2f2f2",
                                        },
                                        {
                                            "lightness": 19,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "administrative",
                                    "elementType": "geometry.fill",
                                    "stylers": [
                                        {
                                            "color": "#fefefe",
                                        },
                                        {
                                            "lightness": 20,
                                        },
                                    ],
                                },
                                {
                                    "featureType": "administrative",
                                    "elementType": "geometry.stroke",
                                    "stylers": [
                                        {
                                            "color": "#fefefe",
                                        },
                                        {
                                            "lightness": 17,
                                        },
                                        {
                                            "weight": 1.2,
                                        },
                                    ],
                                },
                            ],
                        });
                    google.maps.event.addDomListener(window, "resize", function () {
                        let center = map.getCenter();
                        google.maps.event.trigger(map, "resize");
                        map.setCenter(center);
                    });

                    let marker = new google.maps.Marker({
                        position: letLeng,
                        animation: google.maps.Animation.BOUNCE,
                        icon: dsnParam.map.marker_icon,
                        title: "ASL",
                        map: map,

                    });
                });

            } catch (e) {
                console.log(e);
            }
        }, 1000);

    }

    async function linkRightPaginate() {
        const $cont = $('.dsn-paginate-right-page');
        if (!$cont.length) return;
        $cont.empty();
        $('[data-dsn-title]').each(function () {
            const $title = dsnGrid.getData(this, 'title'),
                _target = $(this).offset().top,
                _element = $('<div class="dsn-link-paginate text-transform-upper"></div>');
            _element.html($title);
            $cont.append(_element);
            _element.on('click', function () {
                dsnGrid.scrollTop(_target, 0.85, -150);
            });

        });

    }

    /**
     *  -   event will be triggered by doing browser action such as
     *  a click on the back or forward button
     */
    function effectBackForward() {
        $wind.on("popstate", function (e) {
            if (window.location.hash.length) {
                $wind.scrollTop(0);
                dsnGrid.scrollTop(window.location.hash, 1, -100);
                return;
            }
            if (document.location.href.indexOf("#") > -1) {
                return;
            }
            setTimeout(function () {
                dsnAjax().backAnimate(document.location);
            }, 100);
        });
    }

    function slider($el = $(document)) {
        const dsn_slider = $el.find(".main-slider");
        var tl = gsap.timeline();
        var tl_description = gsap.timeline();

        return {
            run: async function () {
                const $slidObject = this;


                await dsn_slider.each(function () {
                    const $this = $(this),
                        horizontal = $this.hasClass("has-horizontal"),
                        isWebgl = $this.hasClass('dsn-webgl');

                    if (!isWebgl) {
                        $slidObject.getSideSwiper($slidObject, $this, horizontal);
                    }

                });


            },

            getSideSwiper: function ($slidObject, $this, horizontal) {
                $slidObject.initSlider($this)
                    .then(function () {
                        const swiper = $slidObject.swiperObject($this, !horizontal);


                        /**
                         * On Slider Change
                         */
                        $slidObject.slideChange(swiper, $this, horizontal ? "x" : "y");

                        /**
                         *  Add Swiper In Object
                         */
                        dsnGrid.addSwiper(swiper);

                        /**
                         * Arrow Next Slider
                         */
                        $this.find('.next-container').on("click", function () {
                            if (tl.isActive()) return;
                            if (swiper.slides.length === (swiper.activeIndex + 1)) {
                                swiper.slideTo(0);
                            } else {
                                swiper.slideNext();
                            }

                        });


                        /**
                         * Arrow Prev Slider
                         */
                        $this.find('.prev-container').on("click", function () {
                            if (tl.isActive()) return;
                            if (swiper.activeIndex === 0) {
                                swiper.slideTo(swiper.slides.length);
                            } else {
                                swiper.slidePrev();
                            }
                        });


                    });
            },


            /**
             *
             * @param dsn_slider
             *
             */
            initSlider: async function (dsn_slider) {
                await dsn_slider.find(".dsn-slider-content .slide-content [data-dsn-split=\"chars\"]").each(function ($index) {
                    dsnGrid.convertTextLine(this, 'chars', true);
                });
            },

            /**
             *
             * @param dsn_slider
             * @param isDemo
             * @param $isVertical
             * @returns Swiper Slider
             *
             *  Cerate Slider Swiper
             *
             */
            swiperObject: function (dsn_slider, $isVertical = true) {

                let $is_half_center = dsn_slider.hasClass('half-center-slider');

                let $option = $.extend(true, dsnGrid.getData(dsn_slider, 'option', {}),
                    {
                        direction: $isVertical ? "vertical" : "horizontal",
                        slidesPerView: $is_half_center ? "auto" : 1,
                        centeredSlides: true,
                        autoHeight: false,
                        breakpoints: {
                            0: {
                                direction: $isVertical ? "vertical" : "horizontal",
                                slidesPerView: $is_half_center ? "auto" : 1,

                            },
                            767: {
                                direction: $isVertical ? "vertical" : "horizontal",
                                slidesPerView: $is_half_center ? "auto" : 1,


                            },
                            991: {
                                direction: $isVertical ? "vertical" : "horizontal",
                                slidesPerView: $is_half_center ? "auto" : 1,


                            }
                        },
                        on: {
                            init: function () {
                                this.autoplay.stop();

                                dsn_slider.find(".slide-item:not(.swiper-slide-active) [data-dsn=\"video\"] video").each(function () {
                                    this.pause();
                                });
                                this.touchEventsData.formElements = "*";

                            },

                            touchStart: function () {

                                $(this.slides).css("transition", "");
                                if (!dsnGrid.isMobile() && $body.hasClass('dsn-cursor-effect')) {
                                    if (!$(this.slides).parents('.main-slider').hasClass('has-horizontal')) {
                                        $('.cursor').addClass('cursor-scale-half cursor-up-down cursor-drag cursor-next cursor-prev');
                                    } else {
                                        $('.cursor').addClass('cursor-scale-half cursor-drag cursor-next cursor-prev');
                                    }
                                }
                            },
                            touchEnd: function () {

                                if (!dsnGrid.isMobile() && $body.hasClass('dsn-cursor-effect')) {
                                    if (!$(this.slides).hasClass('has-horizontal')) {
                                        $('.cursor').removeClass('cursor-scale-half cursor-up-down cursor-drag cursor-next cursor-prev');
                                    } else {
                                        $('.cursor').removeClass('cursor-scale-half cursor-drag cursor-next cursor-prev');
                                    }
                                }
                            }


                        },
                        pagination: {
                            el: dsn_slider.find(".slider-current-index").get(0),
                            type: "custom",
                            clickable: true,

                            renderCustom: function (swiper, current, total) {
                                dsn_slider.find(".slider-total-index").html(dsnGrid.numberText(total));
                                return dsnGrid.numberText(current);
                            },
                        },
                    }
                );


                return new Swiper(dsn_slider.find(".swiper-container").get(0), $option);
            },
            /**
             *
             * @param swiper
             * @param dsn_slider
             * Change To current Slider
             */
            slideChange: function (swiper, dsn_slider, $dir) {
                let $this = this;
                swiper.on("slideChange", start);

                async function start() {

                    //--> Slider before change
                    let contentOld = dsn_slider.find(".dsn-slider-content .dsn-active");
                    let numOld = contentOld.data("dsn-id");


                    //--> Slider current change
                    let slider = $(swiper.slides[swiper.activeIndex]);

                    let id = slider.data("dsn-id");
                    if (numOld === id) return;
                    dsn_slider.find("[data-dsn=\"video\"] video").each(function () {
                        this.pause();
                    });
                    let v = $(this.slides[this.activeIndex]).find("[data-dsn=\"video\"] video");
                    if (v.length) v.get(0).play();


                    //--> Content Old
                    let content_letterOld = contentOld.find(".dsn-chars-wrapper");
                    contentOld.removeClass("dsn-active-cat");

                    //--> Content New
                    let contentNew = dsn_slider.find(".dsn-slider-content [data-dsn-id=\"" + id + "\"]");
                    let content_letterNew = contentNew.find(".dsn-chars-wrapper");


                    let $isRight = numOld > id;

                    tl.progress(1);
                    tl = new gsap.timeline();

                    tl_description.progress(1);
                    tl_description = new gsap.timeline();

                    let $animate_old = contentOld.find('.swiper-animate-head');
                    let $animate_new = contentNew.find('.swiper-animate-head');

                    if ($animate_old.length)
                        tl_description.fromTo($animate_old, 0.3, $this.showText($dir).title, $this.hideText($isRight, $dir, 10).title);

                    if ($animate_new.length)
                        tl_description.fromTo($animate_new, 0.8, $this.hideText(!$isRight, $dir, 10).title, $this.showText($dir).title, '-=0.2');


                    tl.fromTo(content_letterOld,
                        0.3,
                        $this.showText($dir).title,

                        $.extend(true, $this.hideText($isRight, $dir).title, {
                            stagger: {
                                amount: 0.5
                            }
                        })
                    );

                    tl.call(function () {
                        dsn_slider.find(".dsn-slider-content .slide-content").removeClass("dsn-active").removeClass("dsn-active-cat");
                        contentNew.addClass("dsn-active");
                        contentNew.addClass("dsn-active-cat");
                    });
                    tl.fromTo(
                        content_letterNew,
                        0.8,
                        $this.hideText(!$isRight, $dir).title,
                        $.extend(true, $this.showText($dir).title, {
                            stagger: {
                                amount: 0.5
                            }
                        })
                    );

                    contentOld = numOld = slider = id = v = content_letterOld = content_letterNew = $isRight = null;

                }
            },


            /**
             *
             * @param $dir
             * @returns {{title: {ease: string, autoAlpha: number, yoyo: boolean, rotation: number, scale: number}}}
             */
            showText: function ($dir) {
                let $obj = {
                    title: {
                        autoAlpha: 1,
                        scale: 1,
                        ease: "back.out(4)",
                        yoyo: true,
                    },
                };

                $obj.title[$dir] = "0%";

                return $obj;
            },
            /**
             *
             * @param $isRigth
             * @param $dir
             * @returns {{title: {ease: string, autoAlpha: number, yoyo: boolean, rotation: number}}}
             */
            hideText: function ($isRigth, $dir, $dir_speed = 40) {
                let $obj = {
                    title: {
                        autoAlpha: 0,
                        ease: "back.in(4)",
                        yoyo: true,
                    },
                };

                $obj.title[$dir] = ($isRigth) ? $dir_speed + "%" : ($dir_speed * -1) + "%";
                return $obj;
            },


            /**
             *
             * @param dsn_slider
             * @param $dir
             * @param current
             * @param oldIndex
             * @param $isRight
             */
            slideChangeWeb: function (dsn_slider, $dir, current, oldIndex, $isRight) {
                let $this = this;


                dsn_slider.find(".control-nav .slider-current-index").html(dsnGrid.numberText(current + 1));
                let contentOld = dsn_slider.find(".dsn-slider-content .dsn-active");
                let content_letterOld = contentOld.find(".dsn-chars-wrapper");

                let contentNew = dsn_slider.find(".dsn-slider-content [data-dsn-id=\"" + current + "\"]");
                let content_letterNew = contentNew.find(".dsn-chars-wrapper");
                dsn_slider.find(".slide-inner").attr("data-overlay", contentNew.data("overlay"));

                tl.progress(1);
                tl = gsap.timeline();

                dsn_slider.find(".dsn-slider-content .slide-content").removeClass("dsn-active").removeClass("dsn-active-cat");
                contentNew.addClass("dsn-active");
                contentNew.addClass("dsn-active-cat");

                tl.staggerFromTo(
                    $isRight ? content_letterOld.toArray().reverse() : content_letterOld,

                    0.3,
                    $this.showText($dir).title,
                    $this.hideText($isRight, $dir).title,
                    0.05,
                    0
                );


                tl.fromTo(
                    content_letterNew,
                    $this.hideText(!$isRight, $dir).title,
                    $this.showText($dir).title,
                    // "-=.1",
                );
            },

            /**
             *
             * @param dsn_slider
             * @returns {Promise<{intensity: *, imgs: [], overlay: [], speedIn: *, displacement: *, easing: *, speedOut: *}>}
             */
            initWebgel: async function (dsn_slider) {

                let imgs = [],
                    overlady = [];
                dsn_slider.find(".dsn-slider-content .slide-content").each(function ($index) {
                    let slide_content = $(this);
                    imgs[$index] = slide_content.data("webgel-src");
                    overlady[$index] = slide_content.data("overlay");
                    slide_content.attr("data-dsn-id", $index);
                    if ($index === 0) slide_content.addClass("dsn-active dsn-active-cat");
                    let title = slide_content.find(".title a");
                    dsnGrid.convertTextLine(title);
                    slide_content = title = null;
                });
                dsn_slider.find(".slide-inner").attr("data-overlay", overlady[0]);

                return {
                    imgs: imgs,
                    overlay: overlady,
                    displacement: dsnGrid.getData(dsn_slider.find(".slide-inner"), "displacement", "assets/img/displacement/8.jpg"),
                    intensity: dsnGrid.getData(dsn_slider.find(".slide-inner"), "intensity", -2),
                    speedIn: dsnGrid.getData(dsn_slider.find(".slide-inner"), "speedIn", 1.2),
                    speedOut: dsnGrid.getData(dsn_slider.find(".slide-inner"), "speedOut", 1.2),
                    easing: dsnGrid.getData(dsn_slider.find(".slide-inner"), "easing", "Expo.easeInOut"),
                };

            },


        };
    }


    /**
     *  - the function that move the cursor of an input element to the end
     *
     * @param $off
     *      $off is true stop event listener
     *
     */
    async function mouseCirMove($off) {
        const $elemnet = $("#dsn_cursor");
        if (dsnGrid.isMobile() || !$body.hasClass("dsn-cursor-effect")) {
            if ($elemnet.length) {
                $elemnet.css("display", "none");
                $body.removeClass("dsn-cursor-effect");
            }
            return;
        }


        if ($off === true) {
            $elemnet.attr("class", "cursor");
            cursorEffect($elemnet);
            return;
        }

        if (!$off) {
            dsnGrid.mouseMove($elemnet, {
                speed: dsnParam.cursor.speed
            });

            cursorEffect($elemnet);
        }


    }


    function cursorEffect($elemnet) {

        dsnGrid.elementHover($elemnet, "a:not(> img):not(.vid) , .dsn-button-sidebar,  button , .mfp-container", "cursor-scale-full");
        dsnGrid.elementHover($elemnet, ".c-hidden , .social-side a", "no-scale");
        dsnGrid.elementHover($elemnet, "[data-cursor=\"open\"]", "cursor-scale-half cursor-open");
        dsnGrid.elementHover($elemnet, "[data-cursor=\"close\"]", "cursor-scale-full cursor-close");
        dsnGrid.elementHover($elemnet, "[data-cursor=\"view\"]", "cursor-scale-full cursor-view");

    }


    /**
     *
     */
    function preloader() {

        const preloader = $("#dsn_preloader"),
            progress_number = preloader.find(".percent"),
            progress_title = preloader.find(".title .text-fill"),
            persent = {value: 0},
            preloader_bar = preloader.find(".preloader-bar"),
            preloader_progress = preloader_bar.find(".preloader-progress");


        const timer = dsnGrid.pageLoad(0, 100, 1000, function (val) {
            progress_number.text(val);
            persent.value = val;
            progress_title.css("clip-path", "inset(" + (100 - val) + "% 0% 0% 0%)");
            preloader_progress.css("width", val + "%");

        });

        if (!preloader.length) {
            effectBackForward();
            reloadAjax().catch($err => {
                console.log($err);
            });
        }


        $wind.on("load", function () {

            clearInterval(timer);
            gsap.timeline()
                .to(persent, 1, {
                    value: 100, onUpdate: function () {
                        progress_number.text(persent.value.toFixed(0));
                        progress_title.css("clip-path", "inset(" + (100 - persent.value) + "% 0% 0% 0%)");
                        preloader_progress.css("width", persent.value + "%");
                    },
                })
                .to(preloader.find('> *'), {y: -30, autoAlpha: 0})
                .call(function () {
                    if (preloader.length) {
                        effectBackForward();
                        reloadAjax().catch($err => {
                            console.log($err);
                        });
                    }
                })
                .set(persent, {value: 0})
                .to(persent, 0.8, {
                    value: 100, onUpdate: function () {
                        preloader.css("clip-path", "inset(" + (persent.value) + "% 0% 0% 0%)");
                    },
                    ease: Power2.easeInOut,
                }, "+=0.5")

                .call(function () {
                    preloader.remove();
                });

        });


    }


    /**
     * Option Style Pages
     */
    function changeStyle() {
        const options = $('#dsn_box_options');
        options.find('.title-mode').each(function () {
            dsnGrid.convertTextLine(this);
        });

        options.find('.day-night').on('click', function () {
            const isDark = $body.hasClass('v-dark'),
                _dark = $('.v-dark'),
                _light = $('.v-light');
            $body.toggleClass('v-dark');

            _dark.removeClass('v-dark').addClass('v-light');
            _light.addClass('v-dark').removeClass('v-light');

            $.ajax({
                url: dsnParam.ajaxStyle,
                type: "post",
                data: {
                    color: isDark ? 'v-light' : 'v-dark',
                    style: "off"
                }

            })

        });

        options.find('.mode-layout').on('click', function () {

            const isLine = $body.hasClass('dsn-line-style');

            $body.toggleClass('dsn-line-style');
            for (let s of $build.swiper) {
                s.update();
            }
            Isotope();


            $.ajax({
                url: dsnParam.ajaxStyle,
                type: "post",
                data: {
                    color: "off",
                    style: isLine ? "0" : "1"
                }
            })

        });


    }


    /**
     * Sticky Bar When Down & Up PAge
     */
    function effctStickyNavBar() {
        var bodyScroll = 0, scrDown = 0;

        $effectScroll.getListener(function (e) {

            if (e.type === "scroll") {
                bodyScroll = $wind.scrollTop();
            } else {
                bodyScroll = e.offset.y;
            }


            if (bodyScroll > 170) {

                if (scrDown < bodyScroll) {
                    $body.addClass('nav-bg').addClass('hide-nav')
                } else {
                    $body.removeClass('hide-nav')
                }
            } else {
                $body.removeClass('nav-bg').removeClass('hide-nav');

            }


            scrDown = bodyScroll;
        });
    }

    function dropHash() {

        $("a[href=\"#\"]").on("click", function (e) {
            e.preventDefault();
        });

        $("[href*=\"#\"]:not([href=\"#\"]):not(.wc-tabs .wc-effect-tab)").on("click", function (e) {
            e.preventDefault();
            let href = $($(this).attr("href"));
            if (!href.length) {
                href = null;
                return false;
            }

            dsnGrid.scrollTop(href.get(0).offsetTop, 1, -100);
            href = null;

        });

        if (window.location.hash.length) {
            $wind.scrollTop(0);
            dsnGrid.scrollTop(window.location.hash, 1, -100);
        }


    }


    function list_project() {
        const $grid = $('#dsn_grid');
        const $list = $('#dsn_list');
        const $product = $('ul.dsn-shop');
        $grid.off('click');
        $grid.on('click', function () {
            return changeState($grid, $list);
        });

        $list.off('click');
        $list.on('click', function () {
            return changeState($list, $grid);
        });

        function changeState(_active, _remove) {
            if (_active.hasClass('active'))
                return false;

            _active.addClass('active');
            _remove.removeClass('active');


            $product.fadeOut(300, function () {
                jQuery(this).toggleClass("list").fadeIn(300)
            });

            return false;
        }

    }


    function inputNumber() {


        $('.quantity').each(function () {
            const $this = $(this),
                input = $this.find('input[type="number"]'),
                btnUp = $this.find('.quantity-up'),
                btnDown = $this.find('.quantity-down'),
                min = input.attr('min') || 1,
                max = input.attr('max') || Number.MAX_VALUE;

            btnUp.off('click');
            btnUp.on('click', ChangeValue.bind(this, true));

            btnDown.off('click');
            btnDown.on('click', ChangeValue.bind(this, false));


            function ChangeValue(_is_plus) {
                const oldValue = parseFloat(input.val());
                let newVal = 0;

                if (_is_plus) {
                    if (oldValue >= max) {
                        newVal = oldValue;
                    } else {
                        newVal = oldValue + 1;
                    }
                } else {
                    if (oldValue <= min) {
                        newVal = oldValue;
                    } else {
                        newVal = oldValue - 1;
                    }
                }

                input.val(newVal);
                input.trigger("change");

            }

        });
    }


}
)(jQuery);


function sidebarOptions() {
document.body.classList.toggle('dsn-show-sidebar');
}

function concatAction() {
document.body.classList.toggle('dsn-show-contact');
}


