if (!window.X_LOGIN_EVENTS) {
  window.X_LOGIN_EVENTS = true;

  htmx.onLoad(() => {
    renderRecaptcha();
  });

  document.addEventListener('submit', (e) => {
    if (e.target.closest('#login-form')) {
      loginLoading();
    }
  });
}

function loginLoading() {
  const btn = document.getElementById('btn-login');
  if (btn) {
    btn.classList.add('is-loading');
  }
}

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
