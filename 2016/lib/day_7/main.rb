require_relative '../util'

class Day7
  def initialize
    @input = Util.read_input(7).split("\n")
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def abba?(str)
    (0..str.length - 3).each do |i|
      return true if str[i] == str[i + 3] && str[i + 1] == str[i + 2] && str[i] != str[i + 1]
    end

    false
  end

  def tls?(address)
    address.scan(/\[(.*?)\]/).each do |hypernet|
      return false if abba?(hypernet[0])
    end

    abba?(address)
  end

  def ssl?(address)
    abas = Set.new
    babs = Set.new

    address.split(/[\[\]]/).each_with_index do |str, i|
      if i.even?
        (0..str.length - 2).each do |i|
          abas.add(str[i..i + 2]) if str[i] == str[i + 2] && str[i] != str[i + 1]
        end
      else
        (0..str.length - 2).each do |i|
          babs.add(str[i + 1] + str[i] + str[i + 1]) if str[i] == str[i + 2] && str[i] != str[i + 1]
        end
      end
    end

    abas.intersect?(babs)
  end

  def problem_one
    @input.filter { |address| tls?(address) }.length
  end

  def problem_two
    @input.filter { |address| ssl?(address) }.length
  end
end

Day7.new.main
