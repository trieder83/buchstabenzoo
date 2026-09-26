using UnityEngine;
using System.Collections.Generic;

public class GameManager : MonoBehaviour
{
    public static GameManager Instance { get; private set; }

    public enum Difficulty { Kindergarten, Grade1, Grade2, Grade3 }

    [Header("Game Settings")]
    public Difficulty currentDifficulty = Difficulty.Kindergarten;

    [Header("References")]
    public Transform zooGround;
    public Camera mainCamera;

    private List<Animal> animals = new List<Animal>();
    private int score;

    void Awake()
    {
        if (Instance != null && Instance != this)
        {
            Destroy(gameObject);
            return;
        }
        Instance = this;
    }

    void Start()
    {
        animals.AddRange(FindObjectsByType<Animal>(FindObjectsSortMode.None));
        Debug.Log($"BuchstabenZoo started! Found {animals.Count} animals. Difficulty: {currentDifficulty}");
    }

    public void AddScore(int points)
    {
        score += points;
        Debug.Log($"Score: {score}");
    }

    public Difficulty GetDifficulty() => currentDifficulty;

    public void RegisterAnimal(Animal animal)
    {
        if (!animals.Contains(animal))
            animals.Add(animal);
    }
}
