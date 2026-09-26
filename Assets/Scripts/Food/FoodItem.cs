using UnityEngine;

public class FoodItem : MonoBehaviour
{
    public string foodName;
    public string displayName; // German name on the box
    public bool isLocked = true; // needs pirate ship key

    public void Unlock()
    {
        isLocked = false;
        Debug.Log($"{displayName} ist jetzt verfügbar!");
    }
}
