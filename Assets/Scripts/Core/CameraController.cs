using UnityEngine;

public class CameraController : MonoBehaviour
{
    [Header("Pan Settings")]
    public float panSpeed = 20f;
    public Vector2 panLimitX = new Vector2(-30f, 30f);
    public Vector2 panLimitZ = new Vector2(-30f, 30f);

    [Header("Zoom Settings")]
    public float zoomSpeed = 5f;
    public float minZoom = 10f;
    public float maxZoom = 40f;

    private Vector3 lastPanPosition;
    private bool isPanning;
    private int panFingerId;

    void Update()
    {
        HandlePan();
        HandleZoom();
    }

    void HandlePan()
    {
        // Two-finger pan or right-click drag
        if (Input.touchCount == 1)
        {
            Touch touch = Input.GetTouch(0);

            if (touch.phase == TouchPhase.Began)
            {
                lastPanPosition = touch.position;
                isPanning = true;
            }
            else if (touch.phase == TouchPhase.Moved && isPanning)
            {
                Vector3 delta = (Vector3)touch.position - lastPanPosition;
                Pan(-delta.x, -delta.y);
                lastPanPosition = touch.position;
            }
            else if (touch.phase == TouchPhase.Ended)
            {
                isPanning = false;
            }
        }

        // Mouse fallback for editor testing
        if (Input.GetMouseButtonDown(1))
        {
            lastPanPosition = Input.mousePosition;
            isPanning = true;
        }
        else if (Input.GetMouseButton(1) && isPanning)
        {
            Vector3 delta = Input.mousePosition - lastPanPosition;
            Pan(-delta.x, -delta.y);
            lastPanPosition = Input.mousePosition;
        }
        else if (Input.GetMouseButtonUp(1))
        {
            isPanning = false;
        }
    }

    void Pan(float deltaX, float deltaZ)
    {
        Vector3 move = new Vector3(deltaX, 0, deltaZ) * panSpeed * Time.deltaTime * 0.1f;
        Vector3 newPos = transform.position + move;
        newPos.x = Mathf.Clamp(newPos.x, panLimitX.x, panLimitX.y);
        newPos.z = Mathf.Clamp(newPos.z, panLimitZ.x, panLimitZ.y);
        transform.position = newPos;
    }

    void HandleZoom()
    {
        // Pinch zoom
        if (Input.touchCount == 2)
        {
            Touch t0 = Input.GetTouch(0);
            Touch t1 = Input.GetTouch(1);

            float prevDist = ((t0.position - t0.deltaPosition) - (t1.position - t1.deltaPosition)).magnitude;
            float curDist = (t0.position - t1.position).magnitude;
            float delta = curDist - prevDist;

            Zoom(delta * 0.01f);
        }

        // Scroll wheel fallback
        float scroll = Input.GetAxis("Mouse ScrollWheel");
        if (Mathf.Abs(scroll) > 0.01f)
        {
            Zoom(scroll * zoomSpeed);
        }
    }

    void Zoom(float delta)
    {
        Camera cam = GetComponent<Camera>();
        if (cam.orthographic)
        {
            cam.orthographicSize = Mathf.Clamp(cam.orthographicSize - delta, minZoom, maxZoom);
        }
        else
        {
            Vector3 pos = transform.position;
            pos.y = Mathf.Clamp(pos.y - delta * zoomSpeed, minZoom, maxZoom);
            transform.position = pos;
        }
    }
}
