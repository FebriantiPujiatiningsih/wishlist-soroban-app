// script.js
let wishlist = [];

function addWishlist() {
  const itemName = document.getElementById("itemName").value;
  const category = document.getElementById("category").value;
  const targetPrice = document.getElementById("targetPrice").value;
  const priority = document.getElementById("priority").value;

  if (!itemName || !category || !targetPrice) {
    alert("Please fill all fields!");
    return;
  }

  const item = {
    id: Date.now(),
    itemName,
    category,
    targetPrice,
    priority,
    isAchieved: false,
  };

  wishlist.push(item);
  renderWishlist();
  clearForm();
}

function renderWishlist() {
  const container = document.getElementById("wishlistContainer");
  container.innerHTML = "";

  wishlist.forEach((item) => {
    const card = document.createElement("div");
    card.className = "wishlist-card";

    card.innerHTML = `
      <span class="badge">${item.priority}</span>
      ${item.isAchieved ? '<span class="badge achieved">Achieved</span>' : ""}
      <h3>${item.itemName}</h3>
      <p><strong>Category:</strong> ${item.category}</p>
      <p><strong>Target Price:</strong> Rp ${Number(item.targetPrice).toLocaleString("id-ID")}</p>

      <div class="card-actions">
        <button onclick="markAchieved(${item.id})">Achieve</button>
        <button onclick="deleteWishlist(${item.id})">Delete</button>
      </div>
    `;

    container.appendChild(card);
  });
}

function markAchieved(id) {
  wishlist = wishlist.map((item) => {
    if (item.id === id) {
      return { ...item, isAchieved: true };
    }
    return item;
  });

  renderWishlist();
}

function deleteWishlist(id) {
  wishlist = wishlist.filter((item) => item.id !== id);
  renderWishlist();
}

function clearForm() {
  document.getElementById("itemName").value = "";
  document.getElementById("category").value = "";
  document.getElementById("targetPrice").value = "";
  document.getElementById("priority").value = "High";
}