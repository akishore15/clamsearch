// Create a new XMLHttpRequest object
const xhr = new XMLHttpRequest();

// Set the request method and URL
xhr.open('POST', 'http://localhost:3000', true);

// Set the request headers
xhr.setRequestHeader('Content-Type', 'application/json');

// Create a new JSON object to send to the rendering engine
const data = {
  element: element.outerHTML,
  query: 'Get the rendering results for this element'
};

// Send the request to the rendering engine
xhr.send(JSON.stringify(data));

// Define a callback function to handle the response
xhr.onload = function() {
  if (xhr.status === 200) {
    // Parse the response as JSON
    const response = JSON.parse(xhr.responseText);

    // Log the response to the console
    console.log(response);

    // Return the response as JSON
    return response;
  } else {
    // Log an error message to the console
    console.error('Error:', xhr.statusText);
  }
};
