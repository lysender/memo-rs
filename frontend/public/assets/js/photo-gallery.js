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
    if (e.target.closest('#btn-edit-photos')) {
      handleToggleMenu();
    }
  });

  htmx.on('PhotoDeletedEvent', handlePhotoDeleted);
}

function handleToggleMenu() {
  const container = document.getElementById('photo-gallery');
  if (container) {
    container.classList.toggle('photo-grid-edit');
  }

  const btn = document.getElementById('btn-edit-photos');
  if (btn) {
    btn.classList.toggle('is-active');
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
