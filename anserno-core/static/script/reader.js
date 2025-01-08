"use strict";

class EpubReader {
  constructor() {
    this.source_path = document.getElementById("reader-source-url-path").getAttribute("href");

    this.source_url = new URL(document.URL);
    this.source_url.pathname = this.source_path;

    this.book = ePub(this.source_url.href, {openAs: "epub"})
    this.book.renderTo("display-panel", {width: "100%", height: "100%"});

    this.display = null;
  }

  get rendition() {
    return this.book?.rendition;
  }

  get currentLocation() {
    return this?.book?.rendition?.location?.start;
  }

  prevPage() {
    this.rendition.prev();
    this.updateFragment();
  }

  nextPage() {
    this.rendition.next();
    this.updateFragment();
  }

  updateFragment() {
    window.location.hash = this.currentLocation?.cfi;
  }

  draw(target) {
    target = target || this.currentLocation?.cfi
    if (target) {
      this.display = this.rendition.display(target);
    } else {
      this.display = this.rendition.display();
    }

    if (this.currentLocation?.cfi && target != this.currentLocation?.cfi) {
      this.updateFragment()
    }
  }
}

let epub_reader;

$(window).ready((_event) => {
  const KBD_ARROW_DIRECTION = {
    rtl: { prev: "ArrowRight", next: "ArrowLeft" },
    ltr: { prev: "ArrowLeft", next: "ArrowRight" },
  };

  epub_reader = new EpubReader();

  epub_reader.book.ready.then((event) => {
    let [
      _manifest, spine, _metadata, _cover, navigation, _resources, _displayOptions
    ] = event;

    $("#toc").html($('<div/>', {
      class: "select",
      html: $("<select/>", {
        id: "toc-selector",
        class: "",
        html: spine.items.map((spine_element, index, _array) => {
          let spine_href = decodeURIComponent(spine_element.href);

          let toc_element = navigation.toc.find((toc_element, _index, _array) => {
            let toc_id = decodeURIComponent(toc_element.id);
            let toc_href = decodeURIComponent(toc_element.href);
            if (
              spine_href == toc_id ||
                spine_href == toc_id.substring(0, toc_id.indexOf('#')) ||
                spine_href == toc_href ||
                spine_href == toc_href.substring(0, toc_href.indexOf('#'))
            )
            {
              return toc_element;
            }
          });

          if (toc_element) {
            let title = toc_element?.label ? toc_element.label.trim() : "Entry " + index;
            return $("<option/>", {
              id: "toc-option-" + spine_element.idref,
              label: title,
              text: title,
              value: decodeURIComponent(spine_element.href),
              'data-section-index': spine_element['index'],
              'data-section-source': spine_element['source'],
            });
          }
        }),
        change: (event) => {
          window.location.hash = event.currentTarget.value;
        },
      })
    }));
  });

  epub_reader.rendition.hooks.content.hooks.push((_contents, view) => {
    if ($(`option[value="${decodeURIComponent(view?.location?.start?.href)}"]`).get(0)) {
      $("#toc-selector").get(0).value = decodeURIComponent(view?.location?.start?.href);
    }
  });

  $(".arrow").on("touchend mouseup", (event) => {
    let $arrow_button = $(event.currentTarget);
    $arrow_button.css({"opacity": 0});
  });

  $(".arrow").on("touchstart mousedown", (event) => {
    let $arrow_button = $(event.currentTarget);
    $arrow_button.css({"opacity": 1});
  });

  $("#prev").on("click", epub_reader.prevPage.bind(epub_reader));
  $("#prev-page").on("click", epub_reader.prevPage.bind(epub_reader));

  $("#next").on("click", epub_reader.nextPage.bind(epub_reader));
  $("#next-page").on("click", epub_reader.nextPage.bind(epub_reader));


  $(window).on("keydown", (event) => {
    let direction = epub_reader.rendition.settings.direction || "ltr";
    switch (event.key) {
    case KBD_ARROW_DIRECTION[direction].prev: epub_reader.prevPage(event); break;
    case KBD_ARROW_DIRECTION[direction].next: epub_reader.nextPage(event); break;
    }
  });

  $(window).on('hashchange', (_event) => {
    let current_location = epub_reader.currentLocation?.cfi;
    let window_location = window.location.hash.slice(1);
    if (window_location != current_location) {
      epub_reader.draw(window_location);
    }
  });

  epub_reader.draw(window.location.hash.slice(1));
});
