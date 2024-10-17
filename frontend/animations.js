// 创建粒子效果
function createParticles() {
  const particleCount = 50;
  const container = document.querySelector('.background');

  for (let i = 0; i < particleCount; i++) {
    const particle = document.createElement('div');
    particle.classList.add('particle');

    const size = Math.random() * 5 + 1;
    particle.style.width = `${size}px`;
    particle.style.height = `${size}px`;

    particle.style.left = `${Math.random() * 100}vw`;
    particle.style.top = `${Math.random() * 100}vh`;

    container.appendChild(particle);

    animateParticle(particle);
  }
}

// 动画粒子
function animateParticle(particle) {
  const duration = Math.random() * 3 + 2;
  const delay = Math.random() * 2;

  particle.animate(
    [
      { transform: 'translate(0, 0)' },
      { transform: `translate(${Math.random() * 200 - 100}px, ${Math.random() * 200 - 100}px)` }
    ],
    {
      duration: duration * 1000,
      delay: delay * 1000,
      iterations: Infinity,
      direction: 'alternate',
      easing: 'ease-in-out'
    }
  );
}

// 添加鼠标跟随效果
function addMouseFollowEffect() {
  const container = document.querySelector('.container');

  container.addEventListener('mousemove', (e) => {
    const x = e.clientX / window.innerWidth - 0.5;
    const y = e.clientY / window.innerHeight - 0.5;

    container.style.transform = `perspective(1000px) rotateY(${x * 10}deg) rotateX(${-y * 10}deg)`;
  });

  container.addEventListener('mouseleave', () => {
    container.style.transform = 'perspective(1000px) rotateY(0deg) rotateX(0deg)';
  });
}

// 初始化动画
function initAnimations() {
  createParticles();
  addMouseFollowEffect();
}

// 当页面加载完成时启动动画
window.addEventListener('load', initAnimations);
