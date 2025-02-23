const tutorialTocToggle = document.querySelector("#tutorial-toc-toggle");

document.querySelectorAll("#tutorial-toc li a").forEach((elem) => {
    // Clicking any of the ToC links closes the ToC
    elem.addEventListener("click", (event) => {
        tutorialTocToggle.checked = false;
    })
});

document.addEventListener("keydown", (event) => {
    // Escape closes the ToC
    if (event.key == "Escape") {
        tutorialTocToggle.checked = false;
    }
});

// Select all <samp> elements that are children of <pre> elements
const codeBlocks = document.querySelectorAll("pre > samp");

// Iterate over each code block
codeBlocks.forEach((codeBlock) => {
  // Create a "Copy" button
  const copyButton = document.createElement("button");
  copyButton.classList.add("copy-button");
  copyButton.textContent = "Copy";

  // Add event listener to copy button
  copyButton.addEventListener("click", () => {
    const codeText = codeBlock.innerText;
    navigator.clipboard.writeText(codeText);
    copyButton.textContent = "Copied!";
    copyButton.classList.add("copy-button-copied");
    copyButton.addEventListener("mouseleave", () => {
        copyButton.textContent = "Copy";
        copyButton.classList.remove('copy-button-copied');
    });
  });

  // Create a container for the copy button and append it to the document
  const buttonContainer = document.createElement("div");
  buttonContainer.classList.add("button-container");
  buttonContainer.appendChild(copyButton);
  codeBlock.parentNode.insertBefore(buttonContainer, codeBlock);

  // Hide the button container by default
  buttonContainer.style.display = "none";

  // Show the button container on hover
  codeBlock.parentNode.addEventListener("mouseenter", () => {
    buttonContainer.style.display = "block";
  });

  codeBlock.parentNode.addEventListener("mouseleave", () => {
    buttonContainer.style.display = "none";
  });
});