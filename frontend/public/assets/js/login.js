function onloadCallbackRecaptcha() {
  renderRecaptcha();
}

function renderRecaptcha() {
  const container = document.getElementById('g-recaptcha');
  if (
    container &&
    container.children.length === 0 &&
    window.grecaptcha &&
    window.grecaptcha.render
  ) {
    const key = container.getAttribute('data-sitekey');
    if (key) {
      window.grecaptcha.render('g-recaptcha', {
        sitekey: key,
      });
    }
  }
}

htmx.onLoad(() => {
  renderRecaptcha();
});
