"use strict";

class CollapsibleManager {
  constructor(host) {
    this.host = host;
  }

  get button() {
    return $(this.host);
  }

  get containerId() {
    return '#' + this.button.data('target');
  }

  get container() {
    return $(this.containerId);
  }

  eventHandler(event) {
    let container = this.container;
    let height = container.height();

    if (height === 0) {
      container.css('display', 'block');
      container.height('auto');
    } else {
      container.css('display', 'none');
      container.height(0);
    }
    this.button.toggleClass("selected")
    this.button.children('i.fas').toggleClass("fa-angle-down fa-angle-up")
  }

  attach() {
    this.button.on("click touch", this.eventHandler.bind(this));
    this.container.css('overflow', 'hidden');
    this.button.trigger("click");
  }
}

$(document).ready((event) => {
  $('[data-action="collapse"]').each((_, element) => {
    new CollapsibleManager(element).attach();
  });
});
