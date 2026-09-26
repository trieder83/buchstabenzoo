using UnityEngine;

public class TouchInputController : MonoBehaviour
{
    public Camera mainCamera;
    public LayerMask interactableLayer;

    private Animal selectedAnimal;
    private bool isDragging;

    void Update()
    {
        HandleTouchInput();
    }

    void HandleTouchInput()
    {
        // Support both touch and mouse (for Linux touchscreen + editor testing)
        if (Input.GetMouseButtonDown(0))
        {
            TrySelect(Input.mousePosition);
        }
        else if (Input.GetMouseButton(0) && isDragging && selectedAnimal != null)
        {
            DragAnimal(Input.mousePosition);
        }
        else if (Input.GetMouseButtonUp(0))
        {
            Release();
        }
    }

    void TrySelect(Vector2 screenPos)
    {
        Ray ray = mainCamera.ScreenPointToRay(screenPos);
        if (Physics.Raycast(ray, out RaycastHit hit, 100f, interactableLayer))
        {
            var animal = hit.collider.GetComponent<Animal>();
            if (animal != null)
            {
                selectedAnimal = animal;
                isDragging = true;
                Debug.Log($"Selected: {animal.displayName}");
                return;
            }

            var food = hit.collider.GetComponent<FoodItem>();
            if (food != null && !food.isLocked)
            {
                Debug.Log($"Picked up: {food.displayName}");
            }
        }
    }

    void DragAnimal(Vector2 screenPos)
    {
        Ray ray = mainCamera.ScreenPointToRay(screenPos);
        Plane groundPlane = new Plane(Vector3.up, Vector3.zero);

        if (groundPlane.Raycast(ray, out float distance))
        {
            Vector3 worldPos = ray.GetPoint(distance);
            worldPos.y = 0.5f; // lift slightly while dragging
            selectedAnimal.transform.position = Vector3.Lerp(
                selectedAnimal.transform.position,
                worldPos,
                Time.deltaTime * 15f
            );
        }
    }

    void Release()
    {
        if (selectedAnimal != null)
        {
            Debug.Log($"Released: {selectedAnimal.displayName}");
        }
        selectedAnimal = null;
        isDragging = false;
    }
}
