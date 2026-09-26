using UnityEngine;

public class Enclosure : MonoBehaviour
{
    [Header("Enclosure Settings")]
    public string enclosureName;
    public Animal.AnimalType acceptedAnimalType;
    public Transform animalSpawnPoint;

    [Header("Visual")]
    public GameObject nameSign; // the sign showing the animal name

    public bool HasCorrectAnimal { get; private set; }

    void OnTriggerEnter(Collider other)
    {
        var animal = other.GetComponent<Animal>();
        if (animal != null && animal.type == acceptedAnimalType)
        {
            HasCorrectAnimal = true;
            animal.PlaceInEnclosure(this);
        }
    }
}
