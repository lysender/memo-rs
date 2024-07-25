if (!window.X_GALLERY_EVENTS) {
  window.X_GALLER_EVENTS = true;

  htmx.onLoad(function () {
    var lightbox = new PhotoSwipeLightbox({
      gallery: '#photo-gallery',
      children: '.photo-item-src',
      // dynamic import is not supported in UMD version
      pswpModule: PhotoSwipe,

      showHideAnimationType: 'none',
      showHideDuration: false,
    });

    lightbox.init();
  });

  document.addEventListener('click', (e) => {
    if (e.target.closest('#menu-edit-photos')) {
      handleToggleMenu();
    }
  });

  document.addEventListener('click', (e) => {
    if (e.target.closest('#btn-album-menu')) {
      handleToggleAlbumMenu();
    }
  });

  htmx.on('PhotoDeletedEvent', handlePhotoDeleted);
}

function handleToggleAlbumMenu() {
  const container = document.getElementById('btn-album-menu');
  if (container) {
    container.classList.toggle('is-active');
  }
}

function handleToggleMenu() {
  const container = document.getElementById('photo-gallery');
  if (container) {
    container.classList.toggle('photo-grid-edit');
  }

  const menu = document.getElementById('menu-edit-photos');
  if (menu) {
    menu.classList.toggle('is-active');
  }

  // Also toggle the trigger button color
  const btn = document.getElementById('btn-album-menu-trigger');
  if (btn) {
    btn.classList.toggle('is-info');
  }
}

function handlePhotoDeleted() {
  const currentNode = document.querySelector('#photos-count-w .current-count');
  const totalNode = document.querySelector('#photos-count-w .total-records');

  if (currentNode && totalNode) {
    const current = Number.parseInt(
      currentNode.innerHTML.toString().trim(),
      10,
    );
    const total = Number.parseInt(totalNode.innerHTML.toString().trim(), 10);

    currentNode.innerText = current - 1;
    totalNode.innerText = total - 1;
  }
}
