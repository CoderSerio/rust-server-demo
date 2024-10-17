function ajax(method, url, data = null) {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.open(method, url);
    xhr.onload = () => resolve(xhr.responseText);
    xhr.onerror = () => reject(xhr.statusText);
    xhr.send(data);
  });
}

document.getElementById('getHello').addEventListener('click', async () => {
  try {
    const result = await ajax('GET', 'http://localhost:3000/api/hello');
    document.getElementById('result').textContent = result;
  } catch (error) {
    console.error('Error:', error);
  }
});

document.getElementById('postEcho').addEventListener('click', async () => {
  const input = document.getElementById('echoInput').value;
  try {
    const result = await ajax('POST', 'http://localhost:3000/api/echo', input);
    document.getElementById('result').textContent = result;
  } catch (error) {
    console.error('Error:', error);
  }
});
